use crate::IdWorkerError;

pub trait IdWorker: Send + Sync {
    fn next_id(&self) -> Result<u64, IdWorkerError>;
}
