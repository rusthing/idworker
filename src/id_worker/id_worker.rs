pub trait IdWorker: Send {
    fn next_id(&self) -> u64;

    fn calc_id(
        timestamp: u64,
        timestamp_shift: u8,
        worker: u16,
        sequence: u32,
        sequence_bits: u8,
    ) -> u64
    where
        Self: Sized,
    {
        timestamp << timestamp_shift | (worker as u64) << sequence_bits | sequence as u64
    }
}
