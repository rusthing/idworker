use criterion::{criterion_group, criterion_main, Criterion};
use idworker::{IdWorkerConfig, IdWorkerGenerator, Mode};
use std::hint::black_box;

fn bench_fast_id_worker_normal(c: &mut Criterion) {
    let config = IdWorkerConfig::builder()
        .mode(Mode::Fastest)
        .epoch(1758159446615)
        .build()
        .expect("Failed to build id worker config");
    let id_worker = IdWorkerGenerator::generate(config).expect("Failed to generate id worker");

    c.bench_function("fast_id_worker_normal", |b| {
        b.iter(|| black_box(id_worker.next_id().expect("Failed to generate id")))
    });
}

fn bench_snowflake_id_worker_normal(c: &mut Criterion) {
    let config = IdWorkerConfig::builder()
        .mode(Mode::Normal)
        .epoch(1758159446615)
        .build()
        .expect("Failed to build id worker config");

    let id_worker = IdWorkerGenerator::generate(config).expect("Failed to generate id worker");

    c.bench_function("snowflake_id_worker_normal", |b| {
        b.iter(|| black_box(id_worker.next_id().expect("Failed to generate id")))
    });
}

fn bench_multiple_workers_normal(c: &mut Criterion) {
    let config1 = IdWorkerConfig::builder()
        .mode(Mode::Normal)
        .epoch(1758159446615)
        .build()
        .expect("Failed to build id worker config");

    let config2 = IdWorkerConfig::builder()
        .mode(Mode::Fastest)
        .epoch(1758159446615)
        .build()
        .expect("Failed to build id worker config");

    let id_worker1 = IdWorkerGenerator::generate(config1).expect("Failed to generate id worker");
    let id_worker2 = IdWorkerGenerator::generate(config2).expect("Failed to generate id worker");

    let id_workers = vec![&id_worker1, &id_worker2];

    c.bench_function("multiple_workers_normal", |b| {
        b.iter(|| {
            for id_worker in &id_workers {
                black_box(id_worker.next_id().expect("Failed to generate id"));
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
