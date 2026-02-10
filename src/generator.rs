use crate::id_worker::IdWorker;
use crate::id_worker_options::{IdWorkerOptions, Mode};
use crate::internal::fast_id_worker::FastIdWorker;
use crate::internal::snowflake_id_worker::SnowflakeIdWorker;
use crate::IdWorkerError;

pub struct IdWorkerGenerator {}
impl IdWorkerGenerator {
    pub fn generate(options: IdWorkerOptions) -> Result<Box<dyn IdWorker>, IdWorkerError> {
        match options.mode {
            Mode::Normal => Ok(Box::new(SnowflakeIdWorker::new(options))),
            _ => Ok(Box::new(FastIdWorker::new(options)?)),
        }
    }
}
