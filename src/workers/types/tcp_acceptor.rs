use super::listener::{DNS_BIND_PORT, forward_task};
use crate::exceptions::SCloudException;
use crate::utils;
use crate::workers::task::{InFlightTask, SCloudWorkerTask};
use crate::workers::{SCloudWorker, WorkerState, WorkerType};
use crate::{log_debug, log_error};
use bytes::Bytes;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

/// DNS-over-TCP messages are framed by a 2-byte big-endian length prefix
/// (RFC 1035 §4.2.2). Reject anything claiming more than the UDP-less ceiling.
const MAX_TCP_MESSAGE_BYTES: usize = 65_535;

/// Entry point for a `TCP_ACCEPTOR` worker.
///
/// Like the UDP listener, each acceptor binds the shared port with
/// `SO_REUSEPORT`, so the accept load spreads across all acceptor workers.
pub async fn run_dns_tcp_acceptor(
    worker: Arc<SCloudWorker>,
    tx: Vec<mpsc::Sender<InFlightTask>>,
) -> Result<(), SCloudException> {
    if tx.is_empty() {
        return Err(SCloudException::SCLOUD_WORKER_TX_NOT_SET);
    }

    let listener = build_reuseport_tcp_listener(DNS_BIND_PORT)?;
    worker.set_state(WorkerState::IDLE);

    loop {
        let (stream, peer) = match listener.accept().await {
            Ok(v) => v,
            Err(e) => {
                log_error!("tcp accept failed: {}", e);
                continue;
            }
        };

        let worker = worker.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = serve_tcp_connection(worker, stream, peer, tx).await {
                log_debug!("tcp connection from {} ended: {:?}", peer, e);
            }
        });
    }
}

/// Read length-prefixed DNS messages from a single TCP connection until the
/// client closes it, forwarding each one into the pipeline.
async fn serve_tcp_connection(
    worker: Arc<SCloudWorker>,
    mut stream: TcpStream,
    peer: SocketAddr,
    tx: Vec<mpsc::Sender<InFlightTask>>,
) -> Result<(), SCloudException> {
    loop {
        let mut len_buf = [0u8; 2];
        match stream.read_exact(&mut len_buf).await {
            Ok(_) => {}
            // Clean EOF or reset: the client is done with this connection.
            Err(_) => return Ok(()),
        }

        let msg_len = u16::from_be_bytes(len_buf) as usize;
        if msg_len == 0 || msg_len > MAX_TCP_MESSAGE_BYTES {
            return Ok(());
        }

        let mut payload = vec![0u8; msg_len];
        if stream.read_exact(&mut payload).await.is_err() {
            return Ok(());
        }

        let permit = match worker.in_flight_sem.clone().try_acquire_owned() {
            Ok(p) => p,
            Err(_) => continue,
        };

        let task = SCloudWorkerTask {
            task_id: utils::uuid::generate_uuid(),
            for_type: WorkerType::TCP_ACCEPTOR,
            for_who: peer,
            payload: Bytes::from(payload),
            attempts: 0,
            max_attempts: 0,
            created_at: SystemTime::now(),
            deadline_timeout: None,
            priority: 0,
            // TODO: TCP reply path — the SENDER worker cannot yet route a
            // response back onto this stream (same gap as the UDP path).
            reply_to: None,
            correlation_id: None,
        };

        let in_flight = InFlightTask {
            task,
            _permit: permit,
        };
        forward_task(in_flight, &tx).await;
    }
}

fn build_reuseport_tcp_listener(port: u16) -> Result<TcpListener, SCloudException> {
    use socket2::{Domain, Protocol, Socket, Type};

    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))
        .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_CREATION_FAILED)?;

    #[cfg(not(target_os = "windows"))]
    socket
        .set_reuse_port(true)
        .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_CREATION_FAILED)?;
    socket
        .set_reuse_address(true)
        .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_CREATION_FAILED)?;
    socket
        .set_nonblocking(true)
        .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_CREATION_FAILED)?;

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port));
    socket
        .bind(&addr.into())
        .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_BIND_FAILED)?;
    socket
        .listen(1024)
        .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_BIND_FAILED)?;

    let std_listener: std::net::TcpListener = socket.into();
    TcpListener::from_std(std_listener)
        .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_CREATION_FAILED)
}
