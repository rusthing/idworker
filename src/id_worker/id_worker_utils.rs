use std::time::{SystemTime, UNIX_EPOCH};

pub struct IdWorkerUtils;

impl IdWorkerUtils {
    pub fn calc_timestamp(epoch: u64) -> u64
    where
        Self: Sized,
    {
        // 获取当前毫秒级时间，并转换为10毫秒单位
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
            / 10;
        now - epoch
    }

    pub fn calc_id(
        timestamp: u64,
        timestamp_shift: u8,
        worker: u16,
        sequence: u32,
        sequence_bits: u8,
    ) -> u64
    where
        Self: Sized,
    {
        timestamp << timestamp_shift | (worker as u64) << sequence_bits | sequence as u64
    }
}
