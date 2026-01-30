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
    pub(crate) use crate::threads::windows::priority::imp;
}

#[cfg(target_os = "linux")]
mod thread {
    pub(crate) use crate::threads::linux::priority;
    pub(crate) use crate::threads::linux::imp;
}

#[cfg(target_os = "macos")]
mod thread {
    pub(crate) use crate::threads::macos::priority;
}

#[cfg(not(any(windows, target_os = "linux", target_os= "macos")))]
mod thread {
    pub(crate) use crate::threads::others::priority;
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
    thread::imp::new(cfg, f)
}

#[allow(unused)]
pub fn set_priority(scope: PriorityScope, p: ThreadPriority) -> std::io::Result<()> {
    thread::priority::imp::set_priority(scope, p)
}