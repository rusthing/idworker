use crate::id_worker::IdWorker;
use crate::id_worker_options::{IdWorkerOptions, Mode};
use crate::internal::fast_id_worker::FastIdWorker;
use crate::internal::snowflake_id_worker::SnowflakeIdWorker;
use crate::IdWorkerError;
use std::sync::Arc;

pub struct IdWorkerGenerator {}
impl IdWorkerGenerator {
    pub fn generate(options: IdWorkerOptions) -> Result<Arc<dyn IdWorker>, IdWorkerError> {
        match options.mode {
            Mode::Normal => Ok(Arc::new(SnowflakeIdWorker::new(options))),
            _ => Ok(Arc::new(FastIdWorker::new(options)?)),
        }
    }
}
