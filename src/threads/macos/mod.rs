mod priority;

pub(crate) mod imp {
    use super::super::SpawnConfig;

    pub fn new<F, T>(cfg: SpawnConfig<'_>, f: F) -> std::thread::JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let mut b = std::thread::Builder::new();
        if let Some(name) = cfg.name {
            b = b.name(name.to_string());
        }
        if let Some(sz) = cfg.stack_size {
            b = b.stack_size(sz);
        }

        b.spawn(f).expect("failed to spawn thread")
    }
}
