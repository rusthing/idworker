use crate::{IdWorker, IdWorkerConfig, IdWorkerError, IdWorkerGenerator, IdWorkerOptions};
use log::debug;
use std::sync::OnceLock;

pub static ID_WORKER: OnceLock<Box<dyn IdWorker>> = OnceLock::new();

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
    ID_WORKER
        .set(id_worker)
        .map_err(|_| IdWorkerError::SetIdWorker())?;
    Ok(())
}
