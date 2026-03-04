use crate::IdWorkerError;

pub trait IdWorker: Send + Sync {
    fn next_id(&self) -> Result<i64, IdWorkerError>;
}
