use crate::IdWorkerError;
use crate::internal::id_worker_utils::IdWorkerUtils;
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

pub const ID_WORKER_CONFIG_KEY: &str = "id-worker";

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(build_method(into = Result<IdWorkerConfig, IdWorkerError>))]
#[serde(default, rename_all = "kebab-case", deny_unknown_fields)]
pub struct IdWorkerConfig {
    /// 模式(默认为普通模式)
    #[builder(default = Mode::Normal)]
    pub mode: Mode,
    /// 基准时间(毫秒时间戳)，默认 1758159446615
    #[builder(default = 1758159446615)]
    pub epoch: u64,
    /// 时间戳精度的十进制指数：时间戳单位 = `10^(3 - precision)` 微秒
    ///
    /// - `1`（默认）：0.1ms，每毫秒 10 个刻度
    /// - `0`：1ms
    /// - `-1`：10ms（很多服务器时钟粒度只有 10-15ms，用这个可以省位宽）
    /// - `-2`：100ms
    ///
    /// 注意 `epoch` 要与本精度同量纲后再配：`epoch` 按「时间戳单位数」直接从
    /// 当前时间里减掉，单位对不上会导致时间戳下溢或偏移。
    /// 改这个值等于改 ID 的时间轴，**上线后不要再动**。
    #[builder(default = 1)]
    pub epoch_precision: i8,
    /// 数据中心ID(默认为0)
    #[builder(default = 0)]
    pub data_center: u8,
    /// 数据中心ID位数(默认为0)
    #[builder(default = 0)]
    pub data_center_bits: u8,
    /// 节点ID
    ///
    /// `None` 表示未指定：直接使用时按 0 处理；由框架（如 robotech）初始化时，
    /// 可据此触发自动分配，从而把 0 号留给真正需要它的实例。
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<u8>,
    /// 节点ID位数(默认为4)
    #[builder(default = 4)]
    pub node_bits: u8,
    /// 序列号位数(默认为12)
    #[builder(default = 12)]
    pub sequence_bits: u8,
}

impl IdWorkerConfig {
    /// 校验配置是否合法。
    ///
    /// 除了各字段自身的范围，还会做**位宽预算**检查：按当前时间算出时间戳位数，
    /// 加上 worker / 序列号后不能越过 `i64::MAX`（主键按 i64 落库，越界会变成负数）。
    ///
    /// builder 的 `build()` 与 [`crate::IdWorkerGenerator::generate`] 都会走这里；
    /// 后者覆盖了「配置从文件反序列化」这条不过 `From` 的路径。
    pub fn validate(&self) -> Result<(), IdWorkerError> {
        let IdWorkerConfig {
            epoch,
            epoch_precision,
            data_center,
            data_center_bits,
            node,
            node_bits,
            sequence_bits,
            ..
        } = self;

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        if *epoch > now {
            Err(IdWorkerError::Config(format!(
                "epoch {epoch} cannot be greater than current timestamp"
            )))?;
        }

        // 时间戳单位 = 10^(3-precision) 微秒；范围收窄到 -3..=3，保证 10^n 不溢出
        if !(-3..=3).contains(epoch_precision) {
            Err(IdWorkerError::Config(format!(
                "epoch_precision {epoch_precision} is out of range (-3..=3)"
            )))?;
        }

        if *data_center_bits > 8 {
            Err(IdWorkerError::Config(format!(
                "data_center_bits {data_center_bits} is out of range for u8"
            )))?;
        }
        let max_data_center = (1u16 << data_center_bits) - 1;
        if *data_center as u16 > max_data_center {
            Err(IdWorkerError::Config(format!(
                "data_center {} is out of range for {} bits",
                data_center, data_center_bits
            )))?;
        }

        if *node_bits > 8 {
            Err(IdWorkerError::Config(format!(
                "node_bits {} is out of range for u8",
                node_bits
            )))?;
        }
        let max_node = (1u16 << node_bits) - 1;
        if let Some(node) = *node
            && node as u16 > max_node
        {
            Err(IdWorkerError::Config(format!(
                "node {} is out of range for {} bits",
                node, node_bits
            )))?;
        }

        let max_sequence_bits = 64 - TIMESTAMP_MIN_BITS - data_center_bits - node_bits;
        if *sequence_bits > max_sequence_bits {
            Err(IdWorkerError::Config(format!(
                "sequence_bits {sequence_bits} cannot be greater than {max_sequence_bits}"
            )))?;
        }

        // 位宽预算：按当前时间算时间戳实际占用的位数。精度越细时间戳越大，
        // 「precision 设成 2/3」这类配置会在这里被挡住，而不是产生负数主键。
        let timestamp_bits =
            64 - IdWorkerUtils::calc_timestamp(*epoch, *epoch_precision)?.leading_zeros();
        let total_bits =
            timestamp_bits + *data_center_bits as u32 + *node_bits as u32 + *sequence_bits as u32;
        if total_bits > 63 {
            Err(IdWorkerError::Config(format!(
                "位宽超出 i64 范围：时间戳 {timestamp_bits} 位 + data_center {data_center_bits} 位 \
                 + node {node_bits} 位 + sequence {sequence_bits} 位 = {total_bits} 位（上限 63）。\
                 请调大 epoch、降低 epoch_precision，或减小各段位宽"
            )))?;
        }

        Ok(())
    }
}

// 将校验逻辑实现在 From trait 中（builder 的 build() 依赖它）
impl From<IdWorkerConfig> for Result<IdWorkerConfig, IdWorkerError> {
    fn from(config: IdWorkerConfig) -> Self {
        config.validate()?;
        Ok(config)
    }
}

impl Default for IdWorkerConfig {
    fn default() -> Self {
        IdWorkerConfig {
            mode: Mode::Normal,
            epoch: 1758159446615,
            epoch_precision: 1,
            data_center: 0,
            data_center_bits: 0,
            node: None,
            node_bits: 4,
            sequence_bits: 12,
        }
    }
}
