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
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
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