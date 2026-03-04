use crate::IdWorkerError;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use typed_builder::TypedBuilder;

/// 时间戳最小位数(时间戳位数是自动计算的，但是至少保留41位，能记录69年，42位是139年，43位是279年)
const TIMESTAMP_MIN_BITS: u8 = 41;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Mode {
    /// 普通模式(优化的雪花算法)
    Normal,
    /// 较快速模式(此模式只在初始化时读取时间戳，而后直接递增顺序号，如果顺序号超过最大值时，则获取当前时间戳来重新计算)
    Faster,
    /// 最快速模式(此模式只在初始化时读取时间戳，而后直接递增顺序号，如果顺序号超过最大值时，则直接将之前的时间戳+1来重新计算)
    Fastest,
}

#[derive(Debug, Serialize, Deserialize, Clone, TypedBuilder)]
#[builder(build_method(into = Result<IdWorkerConfig, IdWorkerError>))]
#[serde(rename_all = "kebab-case")]
pub struct IdWorkerConfig {
    /// 模式(默认为普通模式)
    #[builder(default = Mode::Normal)]
    pub mode: Mode,
    /// 基准时间(这个是基于1ms为1个单位，默认为1758159446615)
    #[builder(default = 1758159446615)]
    pub epoch: u64,
    /// 基准时间的时间精度(默认为1)
    /// 很多服务器的系统时钟实际精度只有 10-15 毫秒，使用 10 毫秒级可以减小时间戳的值，节省占位，这时就可以设为-2
    #[builder(default = 1)]
    pub epoch_precision: i8,
    /// 数据中心ID(默认为0)
    #[builder(default = 0)]
    pub data_center: u8,
    /// 数据中心ID位数(默认为0)
    #[builder(default = 0)]
    pub data_center_bits: u8,
    /// 节点ID(默认为0)
    #[builder(default = 0)]
    pub node: u8,
    /// 节点ID位数(默认为4)
    #[builder(default = 4)]
    pub node_bits: u8,
    /// 序列号位数(默认为12)
    #[builder(default = 12)]
    pub sequence_bits: u8,
}

// 将校验逻辑实现在 From trait 中
impl From<IdWorkerConfig> for Result<IdWorkerConfig, IdWorkerError> {
    fn from(config: IdWorkerConfig) -> Self {
        let IdWorkerConfig {
            epoch,
            data_center,
            data_center_bits,
            node,
            node_bits,
            sequence_bits,
            ..
        } = config;

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        if epoch > now {
            Err(IdWorkerError::Config(format!(
                "epoch {epoch} cannot be greater than current timestamp"
            )))?;
        }

        if data_center_bits > 8 {
            Err(IdWorkerError::Config(format!(
                "data_center_bits {data_center_bits} is out of range for u8"
            )))?;
        }
        let max_data_center = (1u16 << data_center_bits) - 1;
        if data_center as u16 > max_data_center {
            Err(IdWorkerError::Config(format!(
                "data_center {} is out of range for {} bits",
                data_center, data_center_bits
            )))?;
        }

        if node_bits > 8 {
            Err(IdWorkerError::Config(format!(
                "node_bits {} is out of range for u8",
                node_bits
            )))?;
        }
        let max_node = (1u16 << node_bits) - 1;
        if node as u16 > max_node {
            Err(IdWorkerError::Config(format!(
                "node {} is out of range for {} bits",
                node, node_bits
            )))?;
        }

        let max_sequence_bits = 64 - TIMESTAMP_MIN_BITS - data_center_bits - node_bits;
        if sequence_bits > max_sequence_bits {
            Err(IdWorkerError::Config(format!(
                "sequence_bits {} cannot be greater than {}",
                sequence_bits, max_sequence_bits
            )))?;
        }

        Ok(config)
    }
}

impl Default for IdWorkerConfig {
    fn default() -> Self {
        IdWorkerConfig::builder().build().unwrap() // 默认值不会报错
    }
}
