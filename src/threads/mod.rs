use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(windows)]
mod windows;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(not(any(windows, target_os = "linux")))]
mod others;

#[cfg(windows)]
mod thread {
    pub(crate) use crate::threads::windows::priority::imp as priority;
    pub(crate) use crate::threads::windows::imp as thread_base;
}

#[cfg(target_os = "linux")]
mod thread {
    pub(crate) use crate::threads::linux::priority::imp as priority;
    pub(crate) use crate::threads::linux::imp as thread_base;
}

#[cfg(target_os = "macos")]
mod thread {
    pub(crate) use crate::threads::macos::priority::imp as priority;
    pub(crate) use crate::threads::macos::imp as thread_base;
}

#[cfg(not(any(windows, target_os = "linux", target_os= "macos")))]
mod thread {
    pub(crate) use crate::threads::others::priority::imp as priority;
    pub(crate) use crate::threads::others::imp as thread_base;
}

#[allow(unused)]
#[allow(non_camel_case_types)]
pub(crate) struct ScloudWorker {
    pub(crate) worker_type: WorkerType,
    pub(crate) os_thread_id: AtomicU64,
    pub(crate) priority: ThreadPriority,
    pub(crate) priority_scope: PriorityScope,
    pub(crate) stack_size_bytes: usize,
    pub(crate) buffer_budget_bytes: usize,
    pub(crate) max_stack_size_bytes: usize,
    pub(crate) max_buffer_budget_bytes: usize,
}

#[allow(unused)]
impl ScloudWorker {
    pub(crate) fn new(worker_type: WorkerType) -> Self {
        Self {
            worker_type,
            os_thread_id: AtomicU64::new(0),
            priority: ThreadPriority::NORMAL,
            priority_scope: PriorityScope::THREAD,
            stack_size_bytes: 2 * 1024 * 1024,
            buffer_budget_bytes: 4 * 1024 * 1024,
            max_stack_size_bytes: 32 * 1024 * 1024,
            max_buffer_budget_bytes: 256 * 1024 * 1024,
        }
    }

    pub(crate) fn set_os_thread_id(&self, tid: u64) {
        self.os_thread_id.store(tid, Ordering::Relaxed);
    }

    pub(crate) fn os_thread_id(&self) -> u64 {
        self.os_thread_id.load(Ordering::Relaxed)
    }
}

#[allow(unused)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WorkerType {
    Listener,
    Decoder,
    QueryDispatcher,
    CacheLookup,
    ZoneManager,
    Resolver,
    CacheWriter,
    Encoder,
    Sender,

    CacheJanitor,

    Metrics,
    TcpAcceptor
}

#[allow(unused)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ThreadPriority {
    IDLE,
    LOW,
    BELOW_NORMAL,
    NORMAL,
    ABOVE_NORMAL,
    HIGH,
    REALTIME,
}

#[allow(unused)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PriorityScope {
    THREAD,
    PROCESS,
    USER,
    PROCESS_GROUP,
}

pub struct SpawnConfig<'a> {
    pub name: Option<&'a str>,
    pub stack_size: Option<usize>,
}

impl<'a> Default for SpawnConfig<'a> {
    fn default() -> Self {
        Self { name: None, stack_size: None }
    }
}

#[allow(unused)]
pub fn new<F, T>(cfg: SpawnConfig<'_>, f: F) -> std::thread::JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    thread::thread_base::new(cfg, f)
}

#[allow(unused)]
pub fn set_priority(scope: PriorityScope, p: ThreadPriority) -> std::io::Result<()> {
    thread::priority::set_priority(scope, p)
}