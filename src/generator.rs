use crate::id_worker::fast_id_worker::FastIdWorker;
use crate::id_worker::id_worker::IdWorker;
use crate::id_worker::snowflake_id_worker::SnowflakeIdWorker;
use crate::options::{Mode, Options};

pub struct IdWorkerGenerator {}
impl IdWorkerGenerator {
    pub fn generate(options: Options) -> Box<dyn IdWorker> {
        match options.mode {
            Mode::Normal => Box::new(SnowflakeIdWorker::new(options)),
            _ => Box::new(FastIdWorker::new(options)),
        }
    }
}
