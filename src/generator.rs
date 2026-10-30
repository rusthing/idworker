use crate::IdWorkerError;
use crate::id_worker::IdWorker;
use crate::id_worker_config::{IdWorkerConfig, Mode};
use crate::internal::fast_id_worker::FastIdWorker;
use crate::internal::snowflake_id_worker::SnowflakeIdWorker;
use std::sync::Arc;

pub struct IdWorkerGenerator {}
impl IdWorkerGenerator {
    pub fn generate(config: IdWorkerConfig) -> Result<Arc<dyn IdWorker>, IdWorkerError> {
        // 配置从文件反序列化时不会走 `From<IdWorkerConfig>`，统一在这里兜一次
        config.validate()?;
        match config.mode {
            Mode::Normal => Ok(Arc::new(SnowflakeIdWorker::new(config))),
            _ => Ok(Arc::new(FastIdWorker::new(config)?)),
        }
    }
}
