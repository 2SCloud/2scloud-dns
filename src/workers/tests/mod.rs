mod types;

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::workers::manager::StartGate;
    use crate::workers::task::InFlightTask;
    use crate::{exceptions, workers};
    use std::any::type_name_of_val;

    use std::sync::Arc;
    use std::sync::atomic::Ordering;
    use tokio::sync::mpsc;

    #[test]
    fn test_init_scloud_worker() {
        let worker = workers::SCloudWorker::new(workers::WorkerType::NONE).unwrap();

        assert_eq!(
            worker.worker_id.load(Ordering::Relaxed),
            worker.get_worker_id()
        );
        assert_eq!(
            workers::WorkerType::try_from(worker.worker_type.load(Ordering::Relaxed)).unwrap(),
            workers::WorkerType::NONE
        );

        assert_eq!(
            worker.stack_size_bytes.load(Ordering::Relaxed),
            2 * 1024 * 1024
        );
        assert_eq!(
            worker.buffer_budget_bytes.load(Ordering::Relaxed),
            4 * 1024 * 1024
        );
        assert_eq!(
            worker.max_stack_size_bytes.load(Ordering::Relaxed),
            32 * 1024 * 1024
        );
        assert_eq!(
            worker.max_buffer_budget_bytes.load(Ordering::Relaxed),
            256 * 1024 * 1024
        );

        assert_eq!(
            worker.state.load(Ordering::Relaxed),
            workers::WorkerState::INIT as u8
        );
        assert!(!worker.shutdown_requested.load(Ordering::Relaxed));
        assert_eq!(
            worker.shutdown_mode.load(Ordering::Relaxed),
            workers::ShutdownMode::GRACEFUL as u8
        );

        assert_eq!(worker.in_flight.load(Ordering::Relaxed), 0);
        assert_eq!(worker.max_in_flight.load(Ordering::Relaxed), 512);

        assert_eq!(worker.jobs_done.load(Ordering::Relaxed), 0);
        assert_eq!(worker.jobs_failed.load(Ordering::Relaxed), 0);
        assert_eq!(worker.jobs_retried.load(Ordering::Relaxed), 0);

        assert_eq!(worker.max_in_flight.load(Ordering::Relaxed), 512);
    }

    #[tokio::test]
    async fn test_run_listener_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::LISTENER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_LISTENER_NO_SOCKET
        ));
    }

    #[tokio::test]
    async fn test_run_listener_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::LISTENER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_LISTENER_NO_SOCKET
        ));
    }

    #[tokio::test]
    async fn test_run_decoder_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::DECODER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        w.dns_tx.lock().await.push(tx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_decoder_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::DECODER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        w.dns_rx.lock().await.push(rx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_query_dispatcher_fails_if_rx_not_set() {
        let w =
            Arc::new(workers::SCloudWorker::new(workers::WorkerType::QUERY_DISPATCHER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        w.dns_tx.lock().await.push(tx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_query_dispatcher_fails_if_tx_not_set() {
        let w =
            Arc::new(workers::SCloudWorker::new(workers::WorkerType::QUERY_DISPATCHER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        w.dns_rx.lock().await.push(rx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_cache_lookup_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::CACHE_LOOKUP).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        w.dns_tx.lock().await.push(tx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_cache_lookup_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::CACHE_LOOKUP).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        w.dns_rx.lock().await.push(rx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_zone_manager_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::ZONE_MANAGER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        w.dns_tx.lock().await.push(tx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_zone_manager_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::ZONE_MANAGER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        w.dns_rx.lock().await.push(rx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_resolver_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::RESOLVER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        w.dns_tx.lock().await.push(tx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_resolver_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::RESOLVER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        w.dns_rx.lock().await.push(rx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_cache_writer_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::CACHE_WRITER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        w.dns_tx.lock().await.push(tx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_cache_writer_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::CACHE_WRITER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        w.dns_rx.lock().await.push(rx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_encoder_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::ENCODER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        w.dns_tx.lock().await.push(tx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_encoder_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::ENCODER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        w.dns_rx.lock().await.push(rx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_sender_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::SENDER).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET
        ));
    }

    #[tokio::test]
    async fn test_run_tcp_acceptor_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        let gate = Arc::new(StartGate::new(1));

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        w.dns_rx.lock().await.push(rx);

        let err = w.clone().run(Some(gate.clone())).await.unwrap_err();
        assert!(matches!(
            err,
            exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET
        ));
    }

    #[test]
    fn test_get_worker_id() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_worker_id(), w.get_worker_id());
        assert_eq!(type_name_of_val(&w.get_worker_id()), "u64");
    }

    #[test]
    fn test_get_worker_type() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(workers::WorkerType::TCP_ACCEPTOR, w.get_worker_type());
    }

    #[tokio::test]
    async fn test_get_dns_rx_tx() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());

        let (tx0, rx0) = mpsc::channel::<InFlightTask>(8);
        w.dns_rx.lock().await.push(rx0);
        w.dns_tx.lock().await.push(tx0);

        let (_rx, _tx) = w.get_dns_rx_tx().await.expect("should return rx+tx");

        assert!(w.dns_rx.lock().await.is_empty());
        assert!(w.dns_tx.lock().await.is_empty());
    }

    #[tokio::test]
    async fn test_get_dns_rx() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());

        let (_, rx0) = mpsc::channel::<InFlightTask>(8);
        w.dns_rx.lock().await.push(rx0);

        let _rx = w.get_dns_rx().await.expect("should return rx");

        assert!(w.dns_rx.lock().await.is_empty());
    }

    #[tokio::test]
    async fn test_get_dns_tx() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());

        let (tx0, _) = mpsc::channel::<InFlightTask>(8);
        w.dns_tx.lock().await.push(tx0);

        let _tx = w.get_dns_tx().await.expect("should return tx");

        assert!(w.dns_tx.lock().await.is_empty());
    }

    #[test]
    pub fn test_get_stack_size_bytes() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_stack_size_bytes(), 2 * 1024 * 1024);
    }

    #[test]
    pub fn test_get_buffer_budget_bytes() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_buffer_budget_bytes(), 4 * 1024 * 1024);
    }

    #[test]
    pub fn test_get_max_stack_size_bytes() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_max_stack_size_bytes(), 32 * 1024 * 1024);
    }

    #[test]
    pub fn test_get_max_buffer_budget_bytes() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_max_buffer_budget_bytes(), 256 * 1024 * 1024);
    }

    #[test]
    pub fn test_get_state() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_state(), workers::WorkerState::INIT as u8);
    }

    #[test]
    pub fn test_get_shutdown_requested() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert!(!w.get_shutdown_requested())
    }

    #[test]
    pub fn test_get_shutdown_mode() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_shutdown_mode(), workers::ShutdownMode::GRACEFUL as u8)
    }

    #[test]
    pub fn test_get_in_flight() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_in_flight(), 0)
    }

    #[test]
    pub fn test_get_in_flight_sem() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_in_flight_sem(), 512)
    }

    #[test]
    pub fn test_get_max_in_flight() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_max_in_flight(), 512)
    }

    #[test]
    pub fn test_get_jobs_done() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_jobs_done(), 0)
    }

    #[test]
    pub fn test_get_jobs_failed() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_jobs_failed(), 0)
    }

    #[test]
    pub fn test_get_jobs_retried() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_jobs_retried(), 0)
    }

    #[test]
    pub fn test_get_last_job_started_ms() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_last_job_started_ms(), 0)
    }

    #[test]
    pub fn test_get_last_job_finished_ms() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_last_job_finished_ms(), 0)
    }

    #[test]
    pub fn test_get_last_error_code() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_last_error_code(), 0)
    }

    #[test]
    pub fn test_get_last_error_at_ms() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_last_error_at_ms(), 0)
    }

    #[test]
    pub fn test_get_last_task_id_hi() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_last_task_id_hi(), 0)
    }

    #[test]
    pub fn test_get_last_task_id_lo() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_last_task_id_lo(), 0)
    }

    #[test]
    pub fn test_set_worker_id() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_worker_id(0);
        assert_eq!(w.get_worker_id(), 0);
    }

    #[test]
    pub fn test_set_worker_type() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_worker_type(workers::WorkerType::LISTENER);
        assert_eq!(w.get_worker_type(), workers::WorkerType::LISTENER);
        w.set_worker_type(workers::WorkerType::DECODER);
        assert_eq!(w.get_worker_type(), workers::WorkerType::DECODER);
        w.set_worker_type(workers::WorkerType::QUERY_DISPATCHER);
        assert_eq!(w.get_worker_type(), workers::WorkerType::QUERY_DISPATCHER);
        w.set_worker_type(workers::WorkerType::CACHE_LOOKUP);
        assert_eq!(w.get_worker_type(), workers::WorkerType::CACHE_LOOKUP);
        w.set_worker_type(workers::WorkerType::ZONE_MANAGER);
        assert_eq!(w.get_worker_type(), workers::WorkerType::ZONE_MANAGER);
        w.set_worker_type(workers::WorkerType::RESOLVER);
        assert_eq!(w.get_worker_type(), workers::WorkerType::RESOLVER);
        w.set_worker_type(workers::WorkerType::CACHE_WRITER);
        assert_eq!(w.get_worker_type(), workers::WorkerType::CACHE_WRITER);
        w.set_worker_type(workers::WorkerType::ENCODER);
        assert_eq!(w.get_worker_type(), workers::WorkerType::ENCODER);
        w.set_worker_type(workers::WorkerType::SENDER);
        assert_eq!(w.get_worker_type(), workers::WorkerType::SENDER);
        w.set_worker_type(workers::WorkerType::CACHE_JANITOR);
        assert_eq!(w.get_worker_type(), workers::WorkerType::CACHE_JANITOR);
        w.set_worker_type(workers::WorkerType::METRICS);
        assert_eq!(w.get_worker_type(), workers::WorkerType::METRICS);
        w.set_worker_type(workers::WorkerType::TCP_ACCEPTOR);
        assert_eq!(w.get_worker_type(), workers::WorkerType::TCP_ACCEPTOR);
        w.set_worker_type(workers::WorkerType::NONE);
        assert_eq!(w.get_worker_type(), workers::WorkerType::NONE);
    }

    #[tokio::test]
    pub async fn test_set_dns_tx() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        let (tx0, _) = mpsc::channel::<InFlightTask>(8);
        w.set_dns_tx(tx0).await;
        assert!(!w.dns_tx.lock().await.is_empty());
    }

    #[tokio::test]
    pub async fn test_set_dns_rx() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        let (_, rx0) = mpsc::channel::<InFlightTask>(8);
        w.set_dns_rx(rx0).await;
        assert!(!w.dns_rx.lock().await.is_empty());
    }

    #[test]
    pub fn test_set_stack_size_bytes() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_stack_size_bytes(64);
        assert_eq!(w.get_stack_size_bytes(), 64);
    }

    #[test]
    pub fn test_set_buffer_budget_bytes() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_buffer_budget_bytes(32);
        assert_eq!(w.get_buffer_budget_bytes(), 32);
    }

    #[test]
    pub fn test_set_max_stack_size_bytes() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_max_stack_size_bytes(32);
        assert_eq!(w.get_max_stack_size_bytes(), 32);
    }

    #[test]
    pub fn test_set_max_buffer_budget_bytes() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_max_buffer_budget_bytes(16);
        assert_eq!(w.get_max_buffer_budget_bytes(), 16);
    }

    #[test]
    pub fn test_set_state() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_state(workers::WorkerState::PAUSED);
        assert_eq!(
            workers::WorkerState::try_from(w.get_state()).unwrap(),
            workers::WorkerState::PAUSED
        );
        w.set_state(workers::WorkerState::INIT);
        assert_eq!(
            workers::WorkerState::try_from(w.get_state()).unwrap(),
            workers::WorkerState::INIT
        );
        w.set_state(workers::WorkerState::IDLE);
        assert_eq!(
            workers::WorkerState::try_from(w.get_state()).unwrap(),
            workers::WorkerState::IDLE
        );
        w.set_state(workers::WorkerState::BUSY);
        assert_eq!(
            workers::WorkerState::try_from(w.get_state()).unwrap(),
            workers::WorkerState::BUSY
        );
        w.set_state(workers::WorkerState::STOPPING);
        assert_eq!(
            workers::WorkerState::try_from(w.get_state()).unwrap(),
            workers::WorkerState::STOPPING
        );
        w.set_state(workers::WorkerState::STOPPED);
        assert_eq!(
            workers::WorkerState::try_from(w.get_state()).unwrap(),
            workers::WorkerState::STOPPED
        );
    }

    #[test]
    pub fn test_set_shutdown_requested() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_shutdown_requested(true);
        assert!(w.get_shutdown_requested());
    }

    #[test]
    pub fn test_set_shutdown_mode() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_shutdown_mode(workers::ShutdownMode::IMMEDIATE);
        assert_eq!(
            workers::ShutdownMode::try_from(w.get_shutdown_mode()).unwrap(),
            workers::ShutdownMode::IMMEDIATE
        );
        w.set_shutdown_mode(workers::ShutdownMode::GRACEFUL);
        assert_eq!(
            workers::ShutdownMode::try_from(w.get_shutdown_mode()).unwrap(),
            workers::ShutdownMode::GRACEFUL
        );
    }

    #[test]
    pub fn test_set_in_flight() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_in_flight(48);
        assert_eq!(w.get_in_flight(), 48);
    }

    #[test]
    pub fn test_set_max_in_flight() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::CACHE_JANITOR).unwrap());
        w.set_max_in_flight(84);
        assert_eq!(w.get_max_in_flight(), 84);
    }

    #[test]
    pub fn test_set_jobs_done() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::METRICS).unwrap());
        w.set_jobs_done(367);
        assert_eq!(w.get_jobs_done(), 367);
    }

    #[test]
    pub fn test_set_jobs_failed() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_jobs_failed(367);
        assert_eq!(w.get_jobs_failed(), 367);
    }

    #[test]
    pub fn test_set_jobs_retried() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_jobs_retried(367);
        assert_eq!(w.get_jobs_retried(), 367);
    }

    #[test]
    pub fn test_set_last_job_started_ms() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_last_job_started_ms(283);
        assert_eq!(w.get_last_job_started_ms(), 283);
    }

    #[test]
    pub fn test_set_last_job_finished_ms() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_last_job_finished_ms(633);
        assert_eq!(w.get_last_job_finished_ms(), 633);
    }

    #[test]
    pub fn test_set_last_error_code() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_last_error_code(39);
        assert_eq!(w.get_last_error_code(), 39);
    }

    #[test]
    pub fn test_set_last_error_at_ms() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_last_error_at_ms(583);
        assert_eq!(w.get_last_error_at_ms(), 583);
    }

    #[test]
    pub fn test_set_last_task_id_hi() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_last_task_id_hi(456);
        assert_eq!(w.get_last_task_id_hi(), 456);
    }

    #[test]
    pub fn test_set_last_task_id_lo() {
        let w = Arc::new(workers::SCloudWorker::new(workers::WorkerType::TCP_ACCEPTOR).unwrap());
        w.set_last_task_id_lo(646);
        assert_eq!(w.get_last_task_id_lo(), 646);
    }
}
