use std::time::{SystemTime, UNIX_EPOCH};

pub enum Mode {
    /// 普通模式(优化的雪花算法)
    Normal,
    /// 较快速模式(此模式只在初始化时读取时间戳，而后直接递增顺序号，如果顺序号超过最大值时，则获取当前时间戳来重新计算)
    Faster,
    /// 最快速模式(此模式只在初始化时读取时间戳，而后直接递增顺序号，如果顺序号超过最大值时，则直接将之前的时间戳+1来重新计算)
    Fastest,
}

pub struct Options {
    /// 模式
    pub mode: Mode,
    /// 基准时间
    pub epoch: u64,
    /// 时间戳位数
    timestamp_bits: u8,
    /// 数据中心ID
    pub data_center: u8,
    /// 数据中心ID位数
    pub data_center_bits: u8,
    /// 节点ID
    pub node: u8,
    /// 节点ID位数
    pub node_bits: u8,
    /// 序列号位数(如果为None，自动计算=64-timestamp_bits-data_center_bits-node_bits)
    pub sequence_bits: u8,
}

/// 默认的基准时间
const EPOCH_DEFAULT: u64 = 1758159446615;
/// 默认时间戳位数
const TIMESTAMP_BITS_DEFAULT: u8 = 42;

impl Options {
    pub fn new() -> Self {
        let data_center_bits = 0;
        let node_bits = 3;
        let timestamp_bits = TIMESTAMP_BITS_DEFAULT;
        Options {
            mode: Mode::Normal,
            epoch: EPOCH_DEFAULT,
            timestamp_bits,
            data_center: 0,
            data_center_bits,
            node: 0,
            node_bits,
            sequence_bits: Self::calc_sequence_bits(timestamp_bits, data_center_bits, node_bits),
        }
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    pub fn epoch(mut self, epoch: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        if epoch > now {
            panic!("epoch {} cannot be greater than current timestamp", epoch);
        }
        self.epoch = epoch;
        self
    }

    pub fn data_center(mut self, data_center: u8, data_center_bits: u8) -> Self {
        if data_center_bits > 8 {
            panic!(
                "data_center_bits {} is out of range for u8",
                data_center_bits
            )
        }
        let max_data_center = (1u16 << data_center_bits) - 1;
        if data_center as u16 > max_data_center {
            panic!(
                "data_center {} is out of range for {} bits",
                data_center, data_center_bits
            );
        }
        self.data_center = data_center;
        self.data_center_bits = data_center_bits;
        self
    }

    pub fn node(mut self, node: u8, node_bits: u8) -> Self {
        if node_bits > 8 {
            panic!("node_bits {} is out of range for u8", node_bits)
        }
        let max_node = (1u16 << node_bits) - 1;
        if node as u16 > max_node {
            panic!("node {} is out of range for {} bits", node, node_bits);
        }
        self.node = node;
        self.node_bits = node_bits;
        self
    }

    pub fn sequence_bits(mut self, sequence_bits: u8) -> Self {
        let max_sequence_bits =
            Self::calc_sequence_bits(self.timestamp_bits, self.data_center_bits, self.node_bits);
        if sequence_bits > max_sequence_bits {
            panic!(
                "sequence_bits {} cannot be greater than {}",
                sequence_bits, max_sequence_bits
            )
        }
        self.sequence_bits = sequence_bits;
        self
    }

    pub fn calc_sequence_bits(timestamp_bits: u8, data_center_bits: u8, node_bits: u8) -> u8 {
        64 - timestamp_bits - data_center_bits - node_bits
    }
}
