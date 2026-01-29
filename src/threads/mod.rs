#[cfg(windows)]
mod windows;
#[cfg(target_os = "linux")]
mod linux;

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

#[allow(unused)]
pub fn set_priority(scope: PriorityScope, p: ThreadPriority) -> std::io::Result<()> {
    imp::set_priority(scope, p)
}

#[cfg(windows)]
use crate::threads::windows::priority::imp;

#[cfg(target_os = "linux")]
use crate::threads::linux::priority::imp;

#[cfg(not(any(windows, target_os = "linux")))]
mod imp {
    use super::{PriorityScope, ThreadPriority};
    use std::io;

    pub(crate) fn set_priority(_: PriorityScope, _: ThreadPriority) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Priority control not supported on this OS",
        ))
    }
}
