use crate::id_worker::IdWorker;
use crate::internal::fast_id_worker::FastIdWorker;
use crate::internal::snowflake_id_worker::SnowflakeIdWorker;
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
