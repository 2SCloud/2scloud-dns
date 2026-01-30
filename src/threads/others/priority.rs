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