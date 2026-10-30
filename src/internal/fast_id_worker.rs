use crate::IdWorkerError;
use crate::id_worker::IdWorker;
use crate::id_worker_config::{IdWorkerConfig, Mode};
use crate::internal::id_worker_utils::IdWorkerUtils;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::warn;

/// 快速模式的 ID 生成器（`Faster` / `Fastest`）。
///
/// 只在初始化时读一次时钟，之后靠 `last_id + 1` 递增；序列号用尽时才推进时间戳：
/// - `Faster`：取当前时间戳（等到它真的比上一次大，避免回拨/精度不足时倒退）；
/// - `Fastest`：直接把上一次的时间戳 +1（可能跑到未来）。
///
/// 两种模式都保证时间戳**严格递增**，因此不会与已发出的 ID 重复；
/// 代价是时间戳可能与真实时间脱钩，多副本部署不要用这两个模式。
pub struct FastIdWorker {
    /// 模式
    mode: Mode,
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
    /// 最大序列号
    max_sequence: u64,
    /// 最后一个ID
    last_id: AtomicU64,
    /// 时钟未前进导致时间戳顺延的次数，仅用于告警限流
    fallback_count: AtomicU64,
}

impl FastIdWorker {
    pub fn new(config: IdWorkerConfig) -> Result<Self, IdWorkerError> {
        let IdWorkerConfig {
            mode,
            epoch,
            epoch_precision,
            data_center,
            data_center_bits,
            node,
            node_bits,
            sequence_bits,
        } = config;

        let timestamp = IdWorkerUtils::calc_timestamp(epoch, epoch_precision)?;
        let worker_bits = data_center_bits + node_bits;
        let worker = ((data_center as u16) << node_bits) + node.unwrap_or_default() as u16;
        let sequence_mask = (1u64 << sequence_bits) - 1;
        let timestamp_shift = worker_bits + sequence_bits;
        let max_sequence = sequence_mask;

        Ok(FastIdWorker {
            mode,
            epoch,
            epoch_precision,
            timestamp_shift,
            worker,
            sequence_bits,
            sequence_mask,
            max_sequence,
            last_id: AtomicU64::new(IdWorkerUtils::calc_id(
                timestamp,
                timestamp_shift,
                worker,
                0,
                sequence_bits,
            )),
            fallback_count: AtomicU64::new(0),
        })
    }
}

impl IdWorker for FastIdWorker {
    fn next_id(&self) -> Result<u64, IdWorkerError> {
        loop {
            let last_id = self.last_id.load(Ordering::Relaxed);
            let last_ts = last_id >> self.timestamp_shift;
            let sequence = last_id & self.sequence_mask;

            let new_id = if sequence < self.max_sequence {
                // 序列号还有余量：低位直接 +1（不会进位到 worker 位）
                last_id + 1
            } else {
                // 序列号用尽，推进时间戳。必须严格大于 last_ts，否则会重号。
                let timestamp = match self.mode {
                    Mode::Faster => {
                        let now = IdWorkerUtils::calc_timestamp(self.epoch, self.epoch_precision)?;
                        if now > last_ts {
                            now
                        } else if IdWorkerUtils::wait_next_timestamp(
                            self.epoch,
                            self.epoch_precision,
                            last_ts,
                        )? {
                            IdWorkerUtils::calc_timestamp(self.epoch, self.epoch_precision)?
                        } else {
                            self.notify_fallback(last_ts);
                            last_ts + 1
                        }
                    }
                    // Mode::Fastest
                    _ => last_ts + 1,
                };
                IdWorkerUtils::calc_id(
                    timestamp,
                    self.timestamp_shift,
                    self.worker,
                    0,
                    self.sequence_bits,
                )
            };

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

impl FastIdWorker {
    /// 时钟没前进时顺延时间戳的告警（限流：每 1024 次一条）。
    fn notify_fallback(&self, last_ts: u64) {
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
    }
}
