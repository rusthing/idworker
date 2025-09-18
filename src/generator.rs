use crate::options::Options;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const TIMESTAMP_BITS: u8 = 42;

pub struct Generator {
    /// 基础时间
    epoch: u64,
    /// 时间戳
    timestamp: u64,
    /// 时间戳需要位移的位数
    timestamp_shift: u8,
    /// 工作节点ID
    worker: u16,
    /// 工作节点ID位数
    worker_bits: u8,
    /// 序列号
    sequence: u32,
    /// 序列号位数
    sequence_bits: u8,
    /// 序列号掩码
    sequence_mask: u64,
    /// 最大序列号
    max_sequence: u32,
    /// 最后一个ID
    last_id: AtomicU64,
}

impl Generator {
    pub fn new(options: Options) -> Self {
        let epoch = options.epoch / 10; // 转换为10毫秒单位
        // 获取当前毫秒级时间，并转换为10毫秒单位
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
            / 10;
        let timestamp = now - epoch;
        let worker_bits = options.data_center_bits + options.node_bits;
        let worker = ((options.data_center as u16) << options.node_bits) + options.node as u16;
        let sequence_bits = 64 - TIMESTAMP_BITS - worker_bits;
        let sequence_mask = (1 << sequence_bits) - 1;
        let timestamp_shift = worker_bits + sequence_bits;
        let max_sequence = (1 << sequence_bits) - 1;

        Generator {
            epoch,
            timestamp,
            timestamp_shift,
            worker,
            worker_bits,
            sequence: 0,
            sequence_bits,
            sequence_mask,
            max_sequence,
            last_id: AtomicU64::new(Generator::calc_id(
                timestamp,
                timestamp_shift,
                worker,
                0,
                sequence_bits,
            )),
        }
    }

    pub fn next_id(&mut self) -> u64 {
        loop {
            let last_id = self.last_id.load(Ordering::Relaxed);
            let sequence = (last_id & self.sequence_mask) as u32;
            let new_id = if sequence == self.max_sequence {
                Generator::calc_id(
                    last_id >> self.timestamp_shift + 1,
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

    fn calc_id(
        timestamp: u64,
        timestamp_shift: u8,
        worker: u16,
        sequence: u32,
        sequence_bits: u8,
    ) -> u64 {
        timestamp << timestamp_shift | (worker as u64) << sequence_bits | sequence as u64
    }
}
