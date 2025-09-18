use std::time::{SystemTime, UNIX_EPOCH};

pub struct Options {
    /// 基准时间
    pub epoch: u64,
    /// 数据中心ID
    pub data_center: u8,
    /// 数据中心ID位数
    pub data_center_bits: u8,
    /// 节点ID
    pub node: u8,
    /// 节点ID位数
    pub node_bits: u8,
}

/// 默认的基准时间
const EPOCH_DEFAULT: u64 = 1758159446615;

impl Options {
    pub fn new() -> Self {
        Options {
            epoch: EPOCH_DEFAULT,
            data_center: 0,
            data_center_bits: 0,
            node: 0,
            node_bits: 3,
        }
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
}
