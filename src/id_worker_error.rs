use std::time::SystemTimeError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum IdWorkerError {
    #[error("配置错误: {0}")]
    Config(String),
    #[error("Fail to set ID_WORKER")]
    SetIdWorker(),
    #[error("Fail to get ID_WORKER")]
    GetIdWorker(),
    #[error("系统时钟错误: {0}")]
    SystemTime(#[from] SystemTimeError),
}
