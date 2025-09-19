use criterion::{criterion_group, criterion_main, Criterion};
use idworker::id_worker::fast_id_worker::FastIdWorker;
use idworker::id_worker::id_worker::IdWorker;
use idworker::id_worker::snowflake_id_worker::SnowflakeIdWorker;
use idworker::options::{Mode, Options};
use std::hint::black_box;

fn bench_fast_id_worker_normal(c: &mut Criterion) {
    let options = Options::new().mode(Mode::Normal).epoch(1758159446615);

    let worker = FastIdWorker::new(options);

    c.bench_function("fast_id_worker_normal", |b| {
        b.iter(|| black_box(worker.next_id()))
    });
}

fn bench_snowflake_id_worker_normal(c: &mut Criterion) {
    let options = Options::new().mode(Mode::Normal).epoch(1758159446615);

    let worker = SnowflakeIdWorker::new(options);

    c.bench_function("snowflake_id_worker_normal", |b| {
        b.iter(|| black_box(worker.next_id()))
    });
}

fn bench_multiple_workers_normal(c: &mut Criterion) {
    let options = Options::new().mode(Mode::Normal).epoch(1758159446615);

    let workers: Vec<Box<dyn IdWorker>> = vec![
        Box::new(FastIdWorker::new(options.clone())),
        Box::new(SnowflakeIdWorker::new(options)),
    ];

    c.bench_function("multiple_workers_normal", |b| {
        b.iter(|| {
            for worker in &workers {
                black_box(worker.next_id());
            }
        })
    });
}

criterion_group!(
    normal_benches,
    bench_fast_id_worker_normal,
    bench_snowflake_id_worker_normal,
    bench_multiple_workers_normal
);

criterion_main!(normal_benches);
