pub trait IdWorker: Send {
    fn next_id(&self) -> u64;
}
