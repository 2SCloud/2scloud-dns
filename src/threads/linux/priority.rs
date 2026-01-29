#[cfg(target_os = "linux")]
pub(crate) mod imp {
    use nix::unistd::{getpgid, getpid, gettid, getuid};
    use std::io;
    use crate::threads::{PriorityScope, ThreadPriority};

    pub(crate) fn set_priority(scope: PriorityScope, p: ThreadPriority) -> io::Result<()> {
        let nice: i32 = match p {
            ThreadPriority::IDLE => 19,
            ThreadPriority::LOW => 10,
            ThreadPriority::BELOW_NORMAL => 5,
            ThreadPriority::NORMAL => 0,
            ThreadPriority::ABOVE_NORMAL => -5,
            ThreadPriority::HIGH => -10,
            ThreadPriority::REALTIME => -20,
        };

        let (which, who) = match scope {
            PriorityScope::THREAD => (libc::PRIO_PROCESS, gettid().as_raw() as libc::id_t),
            PriorityScope::PROCESS => (libc::PRIO_PROCESS, getpid().as_raw() as libc::id_t),
            PriorityScope::USER => (libc::PRIO_USER, getuid().as_raw() as libc::id_t),
            PriorityScope::PROCESS_GROUP => {
                let pgid = getpgid(None).map_err(|e| io::Error::from_raw_os_error(e as i32))?;
                (libc::PRIO_PGRP, pgid.as_raw() as libc::id_t)
            }
        };

        let rc = unsafe { libc::setpriority(which, who, nice) };
        if rc == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}