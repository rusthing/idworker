use crate::IdWorkerError;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub struct IdWorkerUtils;

/// 序列号用尽后等待时钟前进的最长时间。
///
/// 正常情况只需等到下一个时间戳刻度（默认精度 0.1ms）；超时说明时钟停住或发生
/// 回拨，此时必须让时间戳顺延，**不能把序列号回绕**——同一时间戳内回绕会与
/// 已经发出去的 ID 重复。
const WAIT_NEXT_TICK: Duration = Duration::from_millis(2);

impl IdWorkerUtils {
    /// 计算当前时间戳（已减去基准时间）。
    ///
    /// `precision` 是时间戳精度的十进制指数：单位 = `10^(3 - precision)` 微秒。
    ///
    /// | precision | 单位 | 每毫秒刻度数 |
    /// |---|---|---|
    /// | 1（默认） | 0.1ms | 10 |
    /// | 0 | 1ms | 1 |
    /// | -1 | 10ms | 0.1 |
    /// | -2 | 100ms | 0.01 |
    ///
    /// `epoch` 与返回值同量纲的前缀由调用方保证：这里按「时间戳单位数 - epoch」
    /// 计算，所以 `epoch` 也要按同样的单位换算后再配。
    pub fn calc_timestamp(epoch: u64, precision: i8) -> Result<u64, IdWorkerError> {
        let now_us = SystemTime::now().duration_since(UNIX_EPOCH)?.as_micros() as u64;
        // 指数范围由 IdWorkerConfig 校验（-3..=3），这里 clamp 只是兜住直接调用的入参
        let exponent = (3i16 - precision as i16).clamp(0, 18) as u32;
        let now = now_us / 10u64.pow(exponent);
        now.checked_sub(epoch)
            .ok_or(IdWorkerError::TimestampUnderflow { now, epoch })
    }

    /// 等到时间戳超过 `last_ts`。
    ///
    /// 返回 `true` 表示时钟已前进；`false` 表示等待超时（时钟停住或回拨），
    /// 调用方应把时间戳顺延一个刻度。
    pub fn wait_next_timestamp(
        epoch: u64,
        precision: i8,
        last_ts: u64,
    ) -> Result<bool, IdWorkerError> {
        let deadline = Instant::now() + WAIT_NEXT_TICK;
        loop {
            if Self::calc_timestamp(epoch, precision)? > last_ts {
                return Ok(true);
            }
            if Instant::now() >= deadline {
                return Ok(false);
            }
            std::hint::spin_loop();
        }
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
