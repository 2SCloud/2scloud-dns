#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::sync::Arc;

    use tokio::net::UdpSocket;
    use tokio::sync::mpsc;
    use tokio::time::{Duration, timeout};

    use crate::workers;

    fn test_worker(max_in_flight: usize) -> Arc<workers::SCloudWorker> {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::LISTENER).unwrap());
        w.set_max_in_flight(max_in_flight);
        w
    }

    fn exhaust_all_permits(
        worker: &Arc<workers::SCloudWorker>,
    ) -> Vec<tokio::sync::OwnedSemaphorePermit> {
        let mut permits = Vec::new();
        while let Ok(p) = worker.in_flight_sem.clone().try_acquire_owned() {
            permits.push(p);
        }
        permits
    }

    #[tokio::test]
    async fn listener_forwards_udp_payload_into_channel() {
        let worker = test_worker(10);
        let (tx, mut rx) = mpsc::channel::<workers::task::InFlightTask>(8);

        let server = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let server_addr = server.local_addr().unwrap();

        let worker2 = worker.clone();
        let listener = tokio::spawn(async move {
            workers::types::listener::run_dns_listener_with_socket(
                worker2,
                server,
                vec![],
                vec![tx],
            )
            .await
        });

        let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let payload = b"\x01\x02hello-dns";
        client.send_to(payload, server_addr).await.unwrap();

        let in_flight = timeout(Duration::from_millis(500), rx.recv())
            .await
            .expect("timeout while waiting for the task")
            .expect("closed channel");

        assert_eq!(in_flight.task.for_type, workers::WorkerType::LISTENER);
        assert_eq!(in_flight.task.payload.as_ref(), payload);

        let client_addr: SocketAddr = client.local_addr().unwrap();
        assert_eq!(in_flight.task.for_who.ip(), client_addr.ip());

        listener.abort();
    }

    #[tokio::test]
    async fn listener_drops_packets_when_semaphore_exhausted() {
        let worker = test_worker(0);
        assert_eq!(worker.get_max_in_flight(), 0);

        let _guards = exhaust_all_permits(&worker);
        assert_eq!(worker.in_flight_sem.available_permits(), 0);

        let (tx, mut rx) = mpsc::channel::<workers::task::InFlightTask>(8);

        let server = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let server_addr = server.local_addr().unwrap();

        let worker2 = worker.clone();
        let listener = tokio::spawn(async move {
            workers::types::listener::run_dns_listener_with_socket(
                worker2,
                server,
                vec![],
                vec![tx],
            )
            .await
        });

        let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        client.send_to(b"abc", server_addr).await.unwrap();

        match timeout(Duration::from_millis(200), rx.recv()).await {
            Err(_) => {}
            Ok(Some(_)) => {
                panic!("should not have received a task (permit unavailable), but a task arrived")
            }
            Ok(None) => panic!("channel closed (listener dead / tx dropped): invalid test"),
        }

        listener.abort();
    }

    #[tokio::test]
    async fn tcp_acceptor_bind_failure_returns_expected_error() {
        let _worker = test_worker(10);

        use socket2::{Domain, Protocol, Socket, Type};
        use std::net::SocketAddr;

        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();
        #[cfg(not(target_os = "windows"))]
        socket.set_reuse_port(true).unwrap();
        socket.set_nonblocking(true).unwrap();

        let _occupied = UdpSocket::bind("0.0.0.0:0").await.unwrap();
        let occupied_port = _occupied.local_addr().unwrap().port();
        let addr: SocketAddr = format!("0.0.0.0:{}", occupied_port).parse().unwrap();
        let res = socket.bind(&addr.into());
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn listener_recv_on_invalid_socket_fails() {
        use tokio::net::UdpSocket;
        let worker = test_worker(10);
        let (tx, rx) = mpsc::channel::<workers::task::InFlightTask>(1);

        let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();

        let handle = tokio::spawn(workers::types::listener::run_dns_listener_with_socket(
            worker,
            socket,
            vec![rx],
            vec![tx],
        ));
        handle.abort();
        assert!(handle.await.unwrap_err().is_cancelled());
    }
}
