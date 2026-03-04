use crate::id_worker::IdWorker;
use crate::id_worker_config::{IdWorkerConfig, Mode};
use crate::internal::fast_id_worker::FastIdWorker;
use crate::internal::snowflake_id_worker::SnowflakeIdWorker;
use crate::IdWorkerError;
use std::sync::Arc;

pub struct IdWorkerGenerator {}
impl IdWorkerGenerator {
    pub fn generate(config: IdWorkerConfig) -> Result<Arc<dyn IdWorker>, IdWorkerError> {
        match config.mode {
            Mode::Normal => Ok(Arc::new(SnowflakeIdWorker::new(config))),
            _ => Ok(Arc::new(FastIdWorker::new(config)?)),
        }
    }
}
