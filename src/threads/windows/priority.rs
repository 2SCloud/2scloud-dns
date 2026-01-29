#[cfg(windows)]
pub(crate) mod imp {
    use crate::threads::{PriorityScope, ThreadPriority};
    use std::io;

    use winapi::shared::minwindef::FALSE;
    use winapi::um::processthreadsapi::{
        GetCurrentProcess, GetCurrentThread, SetPriorityClass, SetThreadPriority,
    };
    use winapi::um::winbase::*;

    pub(crate) fn set_priority(scope: PriorityScope, p: ThreadPriority) -> io::Result<()> {
        match scope {
            PriorityScope::THREAD => set_thread(p),
            PriorityScope::PROCESS => set_process(p),

            PriorityScope::USER | PriorityScope::PROCESS_GROUP => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Priority scope not supported on Windows",
            )),
        }
    }

    fn set_thread(p: ThreadPriority) -> io::Result<()> {
        let prio = match p {
            ThreadPriority::IDLE => THREAD_PRIORITY_IDLE,
            ThreadPriority::LOW => THREAD_PRIORITY_LOWEST,
            ThreadPriority::BELOW_NORMAL => THREAD_PRIORITY_BELOW_NORMAL,
            ThreadPriority::NORMAL => THREAD_PRIORITY_NORMAL,
            ThreadPriority::ABOVE_NORMAL => THREAD_PRIORITY_ABOVE_NORMAL,
            ThreadPriority::HIGH => THREAD_PRIORITY_HIGHEST,
            ThreadPriority::REALTIME => THREAD_PRIORITY_TIME_CRITICAL,
        };

        let ok = unsafe { SetThreadPriority(GetCurrentThread(), prio) };

        if ok == FALSE {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn set_process(p: ThreadPriority) -> io::Result<()> {
        let class = match p {
            ThreadPriority::IDLE => IDLE_PRIORITY_CLASS,
            ThreadPriority::LOW => BELOW_NORMAL_PRIORITY_CLASS,
            ThreadPriority::BELOW_NORMAL => BELOW_NORMAL_PRIORITY_CLASS,
            ThreadPriority::NORMAL => NORMAL_PRIORITY_CLASS,
            ThreadPriority::ABOVE_NORMAL => ABOVE_NORMAL_PRIORITY_CLASS,
            ThreadPriority::HIGH => HIGH_PRIORITY_CLASS,
            ThreadPriority::REALTIME => REALTIME_PRIORITY_CLASS,
        };

        let ok = unsafe { SetPriorityClass(GetCurrentProcess(), class) };

        if ok == FALSE {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}