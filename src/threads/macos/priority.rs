pub(crate) mod imp {
    use nix::unistd::{getpgid, getpid, getuid};
    use std::io;

    use crate::threads::{PriorityScope, ThreadPriority};

    pub(crate) fn set_priority(scope: PriorityScope, p: ThreadPriority) -> io::Result<()> {
        let want_bg = matches!(
            p,
            ThreadPriority::IDLE | ThreadPriority::LOW | ThreadPriority::BELOW_NORMAL
        );

        match scope {
            PriorityScope::THREAD => {
                let which = libc::PRIO_DARWIN_THREAD;
                let who: libc::id_t = 0;
                let prio: i32 = if want_bg { libc::PRIO_DARWIN_BG } else { 0 };

                let rc = unsafe { libc::setpriority(which, who, prio) };
                if rc == -1 {
                    Err(io::Error::last_os_error())
                } else {
                    Ok(())
                }
            }

            _ => {
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
                    PriorityScope::PROCESS => (libc::PRIO_PROCESS, getpid().as_raw() as libc::id_t),
                    PriorityScope::USER => (libc::PRIO_USER, getuid().as_raw() as libc::id_t),
                    PriorityScope::PROCESS_GROUP => {
                        let pgid = getpgid(None).map_err(|e| match e.as_errno() {
                            Some(errno) => io::Error::from_raw_os_error(errno as i32),
                            None => io::Error::new(io::ErrorKind::Other, e.to_string()),
                        })?;
                        (libc::PRIO_PGRP, pgid.as_raw() as libc::id_t)
                    }

                    PriorityScope::THREAD => unreachable!(),
                };

                let rc = unsafe { libc::setpriority(which, who, nice) };
                if rc == -1 {
                    Err(io::Error::last_os_error())
                } else {
                    Ok(())
                }
            }
        }
    }
}
