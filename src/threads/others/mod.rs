mod priority;

mod imp {
    use super::SpawnConfig;

    pub fn new<F, T>(_: SpawnConfig<'_>, f: F) -> std::thread::JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        std::thread::spawn(f)
    }
}