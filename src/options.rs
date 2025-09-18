pub struct Options {
    /// 基准时间
    pub epoch: u64,
    /// 数据中心ID
    pub data_center: u8,
    /// 数据中心ID位数
    pub data_center_bits: u8,
    /// 工作者ID
    pub worker: u8,
    /// 工作者ID位数
    pub worker_bits: u8,
}

/// 默认的基准时间
const EPOCH_DEFAULT: u64 = 1758159446615;

impl Options {
    pub fn new() -> Self {
        Options {
            epoch: EPOCH_DEFAULT,
            data_center: 0,
            data_center_bits: 0,
            worker: 0,
            worker_bits: 3,
        }
    }

    pub fn epoch(mut self, epoch: u64) -> Self {
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

    pub fn worker(mut self, worker: u8, worker_bits: u8) -> Self {
        if worker_bits > 8 {
            panic!("worker_bits {} is out of range for u8", worker_bits)
        }
        let max_worker = (1u16 << worker_bits) - 1;
        if worker as u16 > max_worker {
            panic!("worker {} is out of range for {} bits", worker, worker_bits);
        }
        self.worker = worker;
        self.worker_bits = worker_bits;
        self
    }
}
