use crate::{IdWorker, IdWorkerConfig, IdWorkerError, IdWorkerGenerator, IdWorkerOptions};
use log::debug;
use std::sync::{Arc, RwLock};

static ID_WORKER: RwLock<Option<Arc<dyn IdWorker>>> = RwLock::new(None);

/// 初始化id生成器
pub fn init_id_worker(id_worker_config: IdWorkerConfig) -> Result<(), IdWorkerError> {
    debug!("init id worker...");
    let id_worker = IdWorkerGenerator::generate(
        IdWorkerOptions::new()
            .epoch(id_worker_config.epoch)?
            .data_center(
                id_worker_config.data_center,
                id_worker_config.data_center_bits,
            )?
            .node(id_worker_config.node, id_worker_config.node_bits)?,
    )?;
    set_id_worker(id_worker)
}

/// 获取当前配置的只读访问
pub fn get_id_worker() -> Result<Arc<dyn IdWorker>, IdWorkerError> {
    let read_lock = ID_WORKER.read().map_err(|_| IdWorkerError::GetIdWorker())?;
    read_lock
        .as_ref()
        .map(Arc::clone)
        .ok_or(IdWorkerError::GetIdWorker())
}

/// 设置配置
pub fn set_id_worker(value: Arc<dyn IdWorker>) -> Result<(), IdWorkerError> {
    let mut write_lock = ID_WORKER
        .write()
        .map_err(|_| IdWorkerError::SetIdWorker())?;
    *write_lock = Some(value);
    Ok(())
}
