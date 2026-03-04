use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub struct IdWorkerUtils;

impl IdWorkerUtils {
    /// 计算时间戳
    /// @param epoch 毫秒级时间戳
    /// @param precision 时间戳精度
    /// 很多服务器的系统时钟实际精度只有 10-15 毫秒，使用 10 毫秒级可以减小时间戳的值，节省占位
    pub fn calc_timestamp(epoch: u64, precision: i8) -> Result<u64, SystemTimeError>
    where
        Self: Sized,
    {
        // 获取当前毫秒级时间，并转换精度
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64
            * (precision * 10) as u64;
        Ok(now - epoch * precision as u64)
    }

    pub fn calc_id(
        timestamp: u64,
        timestamp_shift: u8,
        worker: u16,
        sequence: u32,
        sequence_bits: u8,
    ) -> i64
    where
        Self: Sized,
    {
        (timestamp << timestamp_shift | (worker as u64) << sequence_bits | sequence as u64) as i64
    }
}
