use crate::exceptions::SCloudException;
use crate::workers::manager::StartGate;
use crate::workers::task::InFlightTask;
use crate::{log_error, log_sdebug};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, AtomicUsize, Ordering};
use tokio::sync::{Mutex, Semaphore, mpsc};

pub(crate) mod manager;
pub(crate) mod queue;
pub(crate) mod reply_registry;
pub(crate) mod task;
pub(crate) mod tests;
pub(crate) mod types;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct SCloudWorker {
    // IDENTITY
    pub(crate) worker_id: AtomicU64,
    pub(crate) worker_type: AtomicU8,

    // CHANNEL
    pub(crate) dns_tx: Mutex<Vec<mpsc::Sender<InFlightTask>>>,
    pub(crate) dns_rx: Mutex<Vec<mpsc::Receiver<InFlightTask>>>,

    // RESOURCES/LIMITS
    pub(crate) stack_size_bytes: AtomicUsize,
    pub(crate) buffer_budget_bytes: AtomicUsize,
    pub(crate) max_stack_size_bytes: AtomicUsize,
    pub(crate) max_buffer_budget_bytes: AtomicUsize,

    // RUNTIME STATE
    pub(crate) state: AtomicU8,
    pub(crate) shutdown_requested: AtomicBool,
    pub(crate) shutdown_mode: AtomicU8,

    // BACKPRESSURE/IN-FLIGHT
    pub(crate) in_flight: AtomicUsize, // for metrics
    pub(crate) in_flight_sem: Arc<Semaphore>,
    pub(crate) max_in_flight: AtomicUsize,

    // METRICS
    pub(crate) jobs_done: AtomicU64,
    pub(crate) jobs_failed: AtomicU64,
    pub(crate) jobs_retried: AtomicU64,

    pub(crate) last_job_started_ms: AtomicU64,
    pub(crate) last_job_finished_ms: AtomicU64,

    pub(crate) last_error_code: AtomicU64,
    pub(crate) last_error_at_ms: AtomicU64,

    // CORRELATION/TRACING
    pub(crate) last_task_id_hi: AtomicU64, // 128-bit UUID split
    pub(crate) last_task_id_lo: AtomicU64,
}

impl SCloudWorker {
    const NEVER_APPLIED: u8 = 0xFF;

    pub(crate) fn new(worker_type: WorkerType) -> Result<Self, SCloudException> {
        Ok(Self {
            worker_id: AtomicU64::new(manager::generate_worker_id()),
            worker_type: AtomicU8::new(worker_type as u8),
            dns_tx: Mutex::new(Vec::new()),
            dns_rx: Mutex::new(Vec::new()),
            stack_size_bytes: AtomicUsize::new(2 * 1024 * 1024),
            buffer_budget_bytes: AtomicUsize::new(4 * 1024 * 1024),
            max_stack_size_bytes: AtomicUsize::new(32 * 1024 * 1024),
            max_buffer_budget_bytes: AtomicUsize::new(256 * 1024 * 1024),
            state: AtomicU8::new(WorkerState::INIT as u8),
            shutdown_requested: AtomicBool::new(false),
            shutdown_mode: AtomicU8::new(ShutdownMode::GRACEFUL as u8),
            in_flight: AtomicUsize::new(0),
            in_flight_sem: Arc::new(Semaphore::new(512)),
            max_in_flight: AtomicUsize::new(512),
            jobs_done: AtomicU64::new(0),
            jobs_failed: AtomicU64::new(0),
            jobs_retried: AtomicU64::new(0),
            last_job_started_ms: AtomicU64::new(0),
            last_job_finished_ms: AtomicU64::new(0),
            last_error_code: AtomicU64::new(0),
            last_error_at_ms: AtomicU64::new(0),
            last_task_id_hi: AtomicU64::new(0),
            last_task_id_lo: AtomicU64::new(0),
        })
    }

    pub async fn run(self: Arc<Self>, gate: Option<Arc<StartGate>>) -> Result<(), SCloudException> {
        log_sdebug!(
            "Running SCloudWorker [ID: {}][TYPE: {:?}]",
            self.get_worker_id(),
            self.get_worker_type()
        );

        if let Some(g) = gate {
            g.done().await;
        }
        match WorkerType::try_from(self.worker_type.load(Ordering::Relaxed)).unwrap() {
            WorkerType::LISTENER => {
                self.clone().set_state(WorkerState::IDLE);
                let tx = self.get_dns_tx().await?;
                types::listener::run_dns_listener(self.clone(), tx).await?;
            }
            WorkerType::DECODER => {
                self.clone().set_state(WorkerState::IDLE);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::decoder::run_dns_decoder(self.clone(), rx, tx).await?;
            }
            WorkerType::QUERY_DISPATCHER => {
                self.clone().set_state(WorkerState::IDLE);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::query_dispatcher::run_dns_query_dispatcher(self.clone(), rx, tx).await?;
            }
            WorkerType::CACHE_LOOKUP => {
                self.clone().set_state(WorkerState::IDLE);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::cache_lookup::run_dns_cache_lookup(self.clone(), rx, tx).await?;
            }
            WorkerType::ZONE_MANAGER => {
                self.clone().set_state(WorkerState::IDLE);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::zone_manager::run_dns_zone_manager(self.clone(), rx, tx).await?;
            }
            WorkerType::RESOLVER => {
                self.clone().set_state(WorkerState::IDLE);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::resolver::run_dns_resolver(self.clone(), rx, tx).await?;
            }
            WorkerType::CACHE_WRITER => {
                self.clone().set_state(WorkerState::IDLE);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::cache_writer::run_dns_cache_writer(self.clone(), rx, tx).await?;
            }
            WorkerType::ENCODER => {
                self.clone().set_state(WorkerState::IDLE);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::encoder::run_dns_encoder(self.clone(), rx, tx).await?;
            }
            WorkerType::SENDER => {
                self.clone().set_state(WorkerState::IDLE);
                let rx = self.get_dns_rx().await?;
                types::sender::run_dns_sender(self.clone(), rx).await?;
            }
            WorkerType::CACHE_JANITOR => {
                self.clone().set_state(WorkerState::IDLE);
                types::cache_janitor::run_dns_cache_janitor(self.clone()).await?;
            }
            WorkerType::METRICS => {
                self.clone().set_state(WorkerState::IDLE);
                types::metrics::start_otlp_logger().await;
            }
            WorkerType::TCP_ACCEPTOR => {
                self.clone().set_state(WorkerState::IDLE);
                let tx = self.get_dns_tx().await?;
                types::tcp_acceptor::run_dns_tcp_acceptor(self.clone(), tx).await?;
            }
            WorkerType::DOH_ACCEPTOR => {
                self.clone().set_state(WorkerState::IDLE);
                let tx = self.get_dns_tx().await?;
                types::doh_acceptor::run_dns_doh_acceptor(self.clone(), tx).await?;
            }
            _ => {}
        }
        Ok(())
    }

    #[inline]
    pub fn get_worker_id(&self) -> u64 {
        self.worker_id.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_worker_type(&self) -> WorkerType {
        WorkerType::try_from(self.worker_type.load(Ordering::Relaxed)).unwrap()
    }

    #[inline]
    pub async fn push_dns_rx(&self, rx: mpsc::Receiver<InFlightTask>) {
        self.dns_rx.lock().await.push(rx);
    }

    #[inline]
    pub async fn push_dns_tx_many(&self, txs: Vec<mpsc::Sender<InFlightTask>>) {
        self.dns_tx.lock().await.extend(txs);
    }

    #[inline]
    pub async fn get_dns_rx_tx(
        &self,
    ) -> Result<
        (
            Vec<mpsc::Receiver<InFlightTask>>,
            Vec<mpsc::Sender<InFlightTask>>,
        ),
        SCloudException,
    > {
        Ok((self.get_dns_rx().await?, self.get_dns_tx().await?))
    }

    #[inline]
    pub async fn get_dns_tx(&self) -> Result<Vec<mpsc::Sender<InFlightTask>>, SCloudException> {
        let mut guard = self.dns_tx.lock().await;
        if guard.is_empty() {
            return Err(SCloudException::SCLOUD_WORKER_TX_NOT_SET);
        }
        Ok(std::mem::take(&mut *guard))
    }

    #[inline]
    pub async fn get_dns_rx(&self) -> Result<Vec<mpsc::Receiver<InFlightTask>>, SCloudException> {
        let mut guard = self.dns_rx.lock().await;
        if guard.is_empty() {
            return Err(SCloudException::SCLOUD_WORKER_RX_NOT_SET);
        }
        Ok(std::mem::take(&mut *guard))
    }

    #[inline]
    pub fn get_stack_size_bytes(&self) -> usize {
        self.stack_size_bytes.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_buffer_budget_bytes(&self) -> usize {
        self.buffer_budget_bytes.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_max_stack_size_bytes(&self) -> usize {
        self.max_stack_size_bytes.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_max_buffer_budget_bytes(&self) -> usize {
        self.max_buffer_budget_bytes.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_state(&self) -> u8 {
        self.state.load(Ordering::Acquire)
    }

    #[inline]
    pub fn get_shutdown_requested(&self) -> bool {
        self.shutdown_requested.load(Ordering::Acquire)
    }

    #[inline]
    pub fn get_shutdown_mode(&self) -> u8 {
        self.shutdown_mode.load(Ordering::Acquire)
    }

    #[inline]
    pub fn get_in_flight(&self) -> usize {
        self.in_flight.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_in_flight_sem(&self) -> usize {
        self.in_flight_sem.available_permits()
    }

    #[inline]
    pub fn get_max_in_flight(&self) -> usize {
        self.max_in_flight.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_jobs_done(&self) -> u64 {
        self.jobs_done.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_jobs_failed(&self) -> u64 {
        self.jobs_failed.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_jobs_retried(&self) -> u64 {
        self.jobs_retried.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_job_started_ms(&self) -> u64 {
        self.last_job_started_ms.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_job_finished_ms(&self) -> u64 {
        self.last_job_finished_ms.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_error_code(&self) -> u64 {
        self.last_error_code.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_error_at_ms(&self) -> u64 {
        self.last_error_at_ms.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_task_id_hi(&self) -> u64 {
        self.last_task_id_hi.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_task_id_lo(&self) -> u64 {
        self.last_task_id_lo.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn set_worker_id(&self, worker_id: u64) {
        self.worker_id.store(worker_id, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_worker_type(&self, worker_type: WorkerType) {
        self.worker_type.store(worker_type as u8, Ordering::Relaxed);
    }

    #[inline]
    pub async fn set_dns_tx(&self, tx: mpsc::Sender<InFlightTask>) {
        self.dns_tx.lock().await.push(tx);
    }

    #[inline]
    pub async fn set_dns_rx(&self, rx: mpsc::Receiver<InFlightTask>) {
        self.dns_rx.lock().await.push(rx);
    }

    #[inline]
    pub fn set_stack_size_bytes(&self, stack_size_bytes: usize) {
        self.stack_size_bytes
            .store(stack_size_bytes, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_buffer_budget_bytes(&self, buffer_budget_bytes: usize) {
        self.buffer_budget_bytes
            .store(buffer_budget_bytes, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_max_stack_size_bytes(&self, max_stack_size_bytes: usize) {
        self.max_stack_size_bytes
            .store(max_stack_size_bytes, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_max_buffer_budget_bytes(&self, max_buffer_budget_bytes: usize) {
        self.max_buffer_budget_bytes
            .store(max_buffer_budget_bytes, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_state(&self, state: WorkerState) {
        self.state.store(state as u8, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_shutdown_requested(&self, shutdown_requested: bool) {
        self.shutdown_requested
            .store(shutdown_requested, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_shutdown_mode(&self, shutdown_mode: ShutdownMode) {
        self.shutdown_mode
            .store(shutdown_mode as u8, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_in_flight(&self, in_flight: usize) {
        self.in_flight.store(in_flight, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_max_in_flight(&self, max_in_flight: usize) {
        self.max_in_flight.store(max_in_flight, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_jobs_done(&self, jobs_done: u64) {
        self.jobs_done.store(jobs_done, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_jobs_failed(&self, jobs_failed: u64) {
        self.jobs_failed.store(jobs_failed, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_jobs_retried(&self, jobs_retried: u64) {
        self.jobs_retried.store(jobs_retried, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_job_started_ms(&self, last_job_started_ms: u64) {
        self.last_job_started_ms
            .store(last_job_started_ms, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_job_finished_ms(&self, last_job_finished_ms: u64) {
        self.last_job_finished_ms
            .store(last_job_finished_ms, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_error_code(&self, last_error_code: u64) {
        self.last_error_code
            .store(last_error_code, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_error_at_ms(&self, last_error_at_ms: u64) {
        self.last_error_at_ms
            .store(last_error_at_ms, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_task_id_hi(&self, last_task_id_hi: u64) {
        self.last_task_id_hi
            .store(last_task_id_hi, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_task_id_lo(&self, last_task_id_lo: u64) {
        self.last_task_id_lo
            .store(last_task_id_lo, Ordering::Relaxed);
    }
}

#[repr(u8)]
#[allow(unused)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum WorkerType {
    NONE = 99,
    LISTENER = 0,
    DECODER = 1,
    QUERY_DISPATCHER = 2,
    CACHE_LOOKUP = 3,
    ZONE_MANAGER = 4,
    RESOLVER = 5,
    CACHE_WRITER = 6,
    ENCODER = 7,
    SENDER = 8,

    CACHE_JANITOR = 9,

    METRICS = 10,
    TCP_ACCEPTOR = 11,
    DOH_ACCEPTOR = 12,
}

impl TryFrom<u8> for WorkerType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            0 => WorkerType::LISTENER,
            1 => WorkerType::DECODER,
            2 => WorkerType::QUERY_DISPATCHER,
            3 => WorkerType::CACHE_LOOKUP,
            4 => WorkerType::ZONE_MANAGER,
            5 => WorkerType::RESOLVER,
            6 => WorkerType::CACHE_WRITER,
            7 => WorkerType::ENCODER,
            8 => WorkerType::SENDER,
            9 => WorkerType::CACHE_JANITOR,
            10 => WorkerType::METRICS,
            11 => WorkerType::TCP_ACCEPTOR,
            12 => WorkerType::DOH_ACCEPTOR,
            99 => WorkerType::NONE,
            // TODO: return an SCloudException
            _ => return Err(()),
        })
    }
}

#[repr(u8)]
#[allow(unused)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub(crate) enum WorkerState {
    INIT = 0,
    IDLE = 1,
    BUSY = 2,
    PAUSED = 3,
    STOPPING = 4,
    STOPPED = 5,
}

impl TryFrom<u8> for WorkerState {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            0 => WorkerState::INIT,
            1 => WorkerState::IDLE,
            2 => WorkerState::BUSY,
            3 => WorkerState::PAUSED,
            4 => WorkerState::STOPPING,
            5 => WorkerState::STOPPED,
            // TODO: return an SCloudException
            _ => return Err(()),
        })
    }
}

#[repr(u8)]
#[allow(unused)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub(crate) enum ShutdownMode {
    GRACEFUL = 0,
    IMMEDIATE = 1,
}

impl TryFrom<u8> for ShutdownMode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            0 => ShutdownMode::GRACEFUL,
            1 => ShutdownMode::IMMEDIATE,
            // TODO: return an SCloudException
            _ => return Err(()),
        })
    }
}

pub fn spawn_worker(
    worker: Arc<SCloudWorker>,
    gate: Arc<StartGate>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        gate.wait_turn(worker.get_worker_id()).await;

        if let Err(e) = worker.clone().run(Some(gate.clone())).await {
            log_error!("Worker {} failed: {:?}", worker.get_worker_id(), e);
        }

        gate.done().await;
    })
}
