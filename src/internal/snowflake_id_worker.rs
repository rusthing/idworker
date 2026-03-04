use crate::id_worker::IdWorker;
use crate::id_worker_config::IdWorkerConfig;
use crate::internal::id_worker_utils::IdWorkerUtils;
use crate::IdWorkerError;
use std::sync::atomic::{AtomicU32, Ordering};

/// 雪花ID生成器
/// 仿雪花算法，为提高生成效率，做了如下优化:
/// 1. 将时间戳单位由毫秒调整为10毫秒
/// 2. 不记录上次生成时间戳，直接根据当前时间戳生成id，这样就判断不了时间异常，这个须由运维做好时间同步
/// 3. 序列号从0开始自增1，超出范围的根据掩码取出序列号，溢出32位的自动从0重新开始
pub struct SnowflakeIdWorker {
    /// 基础时间(这个是基于10ms为1个单位)
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
    sequence_mask: u32,
    /// 当前序列号
    sequence: AtomicU32,
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
        let worker = ((data_center as u16) << config.node_bits) + node as u16;
        let sequence_mask = (1 << sequence_bits) - 1;
        let timestamp_shift = worker_bits + sequence_bits;

        SnowflakeIdWorker {
            epoch,
            epoch_precision,
            timestamp_shift,
            worker,
            sequence_bits,
            sequence_mask,
            sequence: AtomicU32::new(0),
        }
    }
}

impl IdWorker for SnowflakeIdWorker {
    fn next_id(&self) -> Result<i64, IdWorkerError> {
        let timestamp = IdWorkerUtils::calc_timestamp(self.epoch, self.epoch_precision)?;
        let sequence = self.sequence.fetch_add(1, Ordering::Relaxed) & self.sequence_mask;
        Ok(IdWorkerUtils::calc_id(
            timestamp,
            self.timestamp_shift,
            self.worker,
            sequence,
            self.sequence_bits,
        ))
    }
}
