use crate::{IdWorker, IdWorkerConfig, IdWorkerError, IdWorkerGenerator};
use arc_swap::ArcSwapOption;
use config::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::debug;

struct IdWorkerRef(Arc<dyn IdWorker>);

const KEY: &str = "id-worker";
static ID_WORKER: ArcSwapOption<IdWorkerRef> = ArcSwapOption::const_empty();

fn get_id_worker() -> Result<Arc<dyn IdWorker>, IdWorkerError> {
    ID_WORKER
        .load_full()
        .map(|r| Arc::clone(&r.0))
        .ok_or(IdWorkerError::GetIdWorker())
}

/// 初始化/更新id生成器
pub fn setup_id_worker(
    id_worker_config: IdWorkerConfig,
    changed: &Option<HashMap<String, Value>>,
) -> Result<(), IdWorkerError> {
    debug!("setup id worker...");
    if changed
        .as_ref()
        .map(|changed| changed.contains_key(KEY))
        .unwrap_or(true)
    {
        let id_worker = IdWorkerGenerator::generate(id_worker_config)?;
        ID_WORKER.store(Some(Arc::new(IdWorkerRef(id_worker))));
    }
    Ok(())
}

/// 生成下一个id
pub fn next_id() -> Result<u64, IdWorkerError> {
    get_id_worker()?.next_id()
}
