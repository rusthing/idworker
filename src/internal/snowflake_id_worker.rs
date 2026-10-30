use crate::IdWorkerError;
use crate::id_worker::IdWorker;
use crate::id_worker_config::IdWorkerConfig;
use crate::internal::id_worker_utils::IdWorkerUtils;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::warn;

/// 雪花ID生成器
///
/// 布局：`时间戳 | worker | 序列号`（各段位宽由配置决定）。
///
/// 与教科书雪花算法的差异：
/// 1. 时间戳精度可配（默认 0.1ms，见 [`IdWorkerUtils::calc_timestamp`]）；
/// 2. 序列号与时间戳绑定：进入新的时间戳刻度时从 0 重新计数。这一点是必须的——
///    若序列号像早期版本那样全局自增、按掩码回绕，回绕时还在同一时间戳内就会把
///    已经发出去的 ID 再发一遍；
/// 3. 同一刻度内序列号用尽时，等待时钟走到下一个刻度；等不到（时钟停住或回拨）
///    则把时间戳顺延一格，宁可让 ID 的绝对时间略微超前，也不产生重复 ID。
pub struct SnowflakeIdWorker {
    /// 基础时间
    epoch: u64,
    /// 时间戳精度
    epoch_precision: i8,
    /// 时间戳需要位移的位数
    timestamp_shift: u8,
    /// 工作节点ID
    worker: u16,
    /// 序列号位数
    sequence_bits: u8,
    /// 序列号掩码
    sequence_mask: u64,
    /// 最近一次发出的 ID，同时承载「时间戳 + 序列号」，用于 CAS 推进
    last_id: AtomicU64,
    /// 时钟未前进导致顺延时间戳的次数，仅用于告警限流
    fallback_count: AtomicU64,
}

impl SnowflakeIdWorker {
    pub fn new(config: IdWorkerConfig) -> Self {
        let IdWorkerConfig {
            epoch,
            epoch_precision,
            data_center,
            data_center_bits,
            node,
            node_bits,
            sequence_bits,
            ..
        } = config;
        let worker_bits = data_center_bits + node_bits;
        let worker = ((data_center as u16) << node_bits) + node.unwrap_or_default() as u16;
        let sequence_mask = (1u64 << sequence_bits) - 1;
        let timestamp_shift = worker_bits + sequence_bits;

        SnowflakeIdWorker {
            epoch,
            epoch_precision,
            timestamp_shift,
            worker,
            sequence_bits,
            sequence_mask,
            // 0 表示「还没发过 ID」：任何真实时间戳都大于它
            last_id: AtomicU64::new(0),
            fallback_count: AtomicU64::new(0),
        }
    }
}

impl IdWorker for SnowflakeIdWorker {
    fn next_id(&self) -> Result<u64, IdWorkerError> {
        loop {
            let last_id = self.last_id.load(Ordering::Relaxed);
            let last_ts = last_id >> self.timestamp_shift;
            let last_seq = last_id & self.sequence_mask;
            let now_ts = IdWorkerUtils::calc_timestamp(self.epoch, self.epoch_precision)?;

            let (ts, seq) = if now_ts > last_ts {
                // 正常的时钟前进：序列号归零
                (now_ts, 0)
            } else if last_seq < self.sequence_mask {
                // 同一刻度内继续递增；时钟回拨时沿用上次的时间戳，靠序列号保持单调
                (last_ts, last_seq + 1)
            } else if IdWorkerUtils::wait_next_timestamp(self.epoch, self.epoch_precision, last_ts)?
            {
                // 等到下一个刻度，回到外层重新计算
                continue;
            } else {
                // 时钟停住/回拨/精度不足：顺延一个刻度，绝不回绕序列号
                if self
                    .fallback_count
                    .fetch_add(1, Ordering::Relaxed)
                    .is_multiple_of(1024)
                {
                    warn!(
                        "id 生成器：时间戳未前进（时钟回拨或系统时钟精度不足 {}），\
                         已顺延到 {}，请检查时间同步",
                        self.epoch_precision,
                        last_ts + 1
                    );
                }
                (last_ts + 1, 0)
            };

            let new_id = IdWorkerUtils::calc_id(
                ts,
                self.timestamp_shift,
                self.worker,
                seq as u32,
                self.sequence_bits,
            );
            if self
                .last_id
                .compare_exchange_weak(last_id, new_id, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                return Ok(new_id);
            }
        }
    }
}
