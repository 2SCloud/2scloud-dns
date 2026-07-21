use crate::exceptions::SCloudException;
use crate::utils;
use crate::workers::task::{InFlightTask, SCloudWorkerTask};
use crate::workers::{SCloudWorker, WorkerState, WorkerType};
use bytes::Bytes;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

/// Port shared by the UDP listener and the TCP acceptor.
pub(crate) const DNS_BIND_PORT: u16 = 5353;

/// Entry point for a UDP `LISTENER` worker.
///
/// Each listener creates its own `SO_REUSEPORT` socket bound to the same
/// address/port. The kernel then load-balances incoming datagrams across all
/// listener sockets, so scaling throughput toward high packet-per-second
/// targets is a matter of running more `LISTENER` workers (`workers.listener`).
pub async fn run_dns_listener(
    worker: Arc<SCloudWorker>,
    tx: Vec<mpsc::Sender<InFlightTask>>,
) -> Result<(), SCloudException> {
    #[cfg(not(target_os = "windows"))]
    {
        let socket = build_reuseport_udp_socket(DNS_BIND_PORT)?;
        run_dns_listener_with_socket(worker, socket, vec![], tx).await
    }

    #[cfg(target_os = "windows")]
    {
        run_dns_listener_with_shared_socket(worker, tx).await
    }
}

/// Build a UDP socket with `SO_REUSEPORT`/`SO_REUSEADDR` so multiple listener
/// workers can bind the same port concurrently.
#[cfg(not(target_os = "windows"))]
fn build_reuseport_udp_socket(port: u16) -> Result<UdpSocket, SCloudException> {
    use socket2::{Domain, Protocol, Socket, Type};
    use std::net::SocketAddr;

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
        .map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)?;

    socket
        .set_reuse_port(true)
        .map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)?;
    socket
        .set_reuse_address(true)
        .map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)?;
    socket
        .set_nonblocking(true)
        .map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)?;
    socket
        .set_recv_buffer_size(16 * 1024 * 1024)
        .map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)?;
    socket
        .set_send_buffer_size(16 * 1024 * 1024)
        .map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)?;

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port));
    socket
        .bind(&addr.into())
        .map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)?;

    let std_socket: std::net::UdpSocket = socket.into();
    UdpSocket::from_std(std_socket).map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)
}

pub async fn run_dns_listener_with_socket(
    worker: Arc<SCloudWorker>,
    socket: UdpSocket,
    _rx: Vec<mpsc::Receiver<InFlightTask>>,
    tx: Vec<mpsc::Sender<InFlightTask>>,
) -> Result<(), SCloudException> {
    let mut buf = [0u8; 65_535];
    worker.set_state(WorkerState::IDLE);
    if tx.is_empty() {
        return Err(SCloudException::SCLOUD_WORKER_TX_NOT_SET);
    }

    loop {
        let (len, src) = socket
            .recv_from(&mut buf)
            .await
            .map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_RECV_FAILED)?;

        let permit = match worker.in_flight_sem.clone().try_acquire_owned() {
            Ok(p) => p,
            Err(_) => continue,
        };

        let task = SCloudWorkerTask {
            task_id: utils::uuid::generate_uuid(),
            for_type: WorkerType::LISTENER,
            for_who: src,
            payload: Bytes::copy_from_slice(&buf[..len]),
            attempts: 0,
            max_attempts: 0,
            created_at: SystemTime::now(),
            deadline_timeout: None,
            priority: 0,
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

#[cfg(target_os = "windows")]
pub async fn run_dns_listener_with_shared_socket(
    worker: Arc<SCloudWorker>,
    tx: Vec<mpsc::Sender<InFlightTask>>,
) -> Result<(), SCloudException> {
    let udp = SHARED_UDP_SOCKET
        .get()
        .ok_or(SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)?
        .clone();

    let mut buf = [0u8; 65_535];
    worker.set_state(WorkerState::IDLE);

    loop {
        let (len, src) = udp
            .recv_from(&mut buf)
            .await
            .map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_RECV_FAILED)?;

        let permit = match worker.in_flight_sem.clone().try_acquire_owned() {
            Ok(p) => p,
            Err(_) => continue,
        };

        let task = SCloudWorkerTask {
            task_id: utils::uuid::generate_uuid(),
            for_type: WorkerType::LISTENER,
            for_who: src,
            payload: Bytes::copy_from_slice(&buf[..len]),
            attempts: 0,
            max_attempts: 0,
            created_at: SystemTime::now(),
            deadline_timeout: None,
            priority: 0,
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

pub(crate) async fn forward_task(task: InFlightTask, tx: &[mpsc::Sender<InFlightTask>]) -> bool {
    let mut current = Some(task);
    for tx_channel in tx.iter() {
        match tx_channel.try_send(current.take().unwrap()) {
            Ok(_) => return true,
            Err(mpsc::error::TrySendError::Full(returned)) => {
                current = Some(returned);
            }
            Err(mpsc::error::TrySendError::Closed(_)) => return false,
        }
    }
    if let Some(unsent) = current
        && let Some(tx_channel) = tx.first()
    {
        return tx_channel.send(unsent).await.is_ok();
    }
    true
}

#[cfg(target_os = "windows")]
pub static SHARED_UDP_SOCKET: std::sync::OnceLock<Arc<UdpSocket>> = std::sync::OnceLock::new();
