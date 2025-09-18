use crate::id_worker::id_worker::IdWorker;
use crate::options::{Mode, Options};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct FastIdWorker {
    /// 模式
    mode: Mode,
    /// 基础时间(这个是基于10ms为1个单位)
    epoch: u64,
    /// 时间戳需要位移的位数
    timestamp_shift: u8,
    /// 工作节点ID
    worker: u16,
    /// 序列号位数
    sequence_bits: u8,
    /// 序列号掩码
    sequence_mask: u64,
    /// 最大序列号
    max_sequence: u32,
    /// 最后一个ID
    last_id: AtomicU64,
}

impl FastIdWorker {
    pub fn new(options: Options) -> Self {
        let epoch = options.epoch / 10; // 转换为10毫秒单位
        let timestamp = Self::calc_timestamp(epoch);
        let worker_bits = options.data_center_bits + options.node_bits;
        let worker = ((options.data_center as u16) << options.node_bits) + options.node as u16;
        let sequence_bits = options.sequence_bits;
        let sequence_mask = (1 << sequence_bits) - 1;
        let timestamp_shift = worker_bits + sequence_bits;
        let max_sequence = (1 << sequence_bits) - 1;

        FastIdWorker {
            mode: options.mode,
            epoch,
            timestamp_shift,
            worker,
            sequence_bits,
            sequence_mask,
            max_sequence,
            last_id: AtomicU64::new(Self::calc_id(
                timestamp,
                timestamp_shift,
                worker,
                0,
                sequence_bits,
            )),
        }
    }
}

impl IdWorker for FastIdWorker {
    fn next_id(&self) -> u64 {
        loop {
            let last_id = self.last_id.load(Ordering::Relaxed);
            let sequence = (last_id & self.sequence_mask) as u32;
            let new_id = if sequence == self.max_sequence {
                let timestamp = match self.mode {
                    Mode::Faster => Self::calc_timestamp(self.epoch) >> self.timestamp_shift + 1,
                    _ => last_id >> self.timestamp_shift + 1,
                };

                FastIdWorker::calc_id(
                    timestamp,
                    self.timestamp_shift,
                    self.worker,
                    0,
                    self.sequence_bits,
                )
            } else {
                last_id + 1
            };
            match self.last_id.compare_exchange_weak(
                last_id,           // 当前值
                new_id,            // 新值
                Ordering::Relaxed, // 成功时的内存顺序
                Ordering::Relaxed, // 失败时的内存顺序
            ) {
                Ok(_) => return new_id, // 成功
                Err(_) => continue,     // 失败（可能是伪失败，性能高），重试
            }
        }
    }
}
