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
    #[error(
        "时间戳下溢: 当前时间 {now} 早于基准时间 {epoch}，请检查 epoch 与 epoch-precision 是否同量纲"
    )]
    TimestampUnderflow { now: u64, epoch: u64 },
}
