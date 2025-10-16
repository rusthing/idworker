use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct IdWorkerSettings {
    /// 基准时间(基于1ms为1个单位)
    #[serde(default = "epoch_default")]
    pub epoch: u64,
    /// 数据中心ID
    #[serde(default = "data_center_default")]
    pub data_center: u8,
    /// 数据中心ID位数
    #[serde(default = "data_center_bits_default")]
    pub data_center_bits: u8,
    /// 节点ID
    #[serde(default = "node_default")]
    pub node: u8,
    /// 节点ID位数
    #[serde(default = "node_bits_default")]
    pub node_bits: u8,
}

impl Default for IdWorkerSettings {
    fn default() -> Self {
        IdWorkerSettings {
            epoch: epoch_default(),
            data_center: data_center_default(),
            data_center_bits: data_center_bits_default(),
            node: node_default(),
            node_bits: node_bits_default(),
        }
    }
}

fn epoch_default() -> u64 {
    1758107692220
}
fn data_center_default() -> u8 {
    0
}
fn data_center_bits_default() -> u8 {
    0
}
fn node_default() -> u8 {
    0
}

fn node_bits_default() -> u8 {
    3
}
