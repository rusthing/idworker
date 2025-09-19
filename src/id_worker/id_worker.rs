pub trait IdWorker: Send + Sync {
    fn next_id(&self) -> u64;
}
