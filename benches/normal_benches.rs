use criterion::{criterion_group, criterion_main, Criterion};
use idworker::{IdWorkerGenerator, Mode, Options};
use std::hint::black_box;

fn bench_fast_id_worker_normal(c: &mut Criterion) {
    let options = Options::new().mode(Mode::Fastest).epoch(1758159446615);
    let id_worker = IdWorkerGenerator::generate(options);

    c.bench_function("fast_id_worker_normal", |b| {
        b.iter(|| black_box(id_worker.next_id()))
    });
}

fn bench_snowflake_id_worker_normal(c: &mut Criterion) {
    let options = Options::new().mode(Mode::Normal).epoch(1758159446615);

    let id_worker = IdWorkerGenerator::generate(options);

    c.bench_function("snowflake_id_worker_normal", |b| {
        b.iter(|| black_box(id_worker.next_id()))
    });
}

fn bench_multiple_workers_normal(c: &mut Criterion) {
    let options1 = Options::new().mode(Mode::Normal).epoch(1758159446615);
    let options2 = Options::new().mode(Mode::Fastest).epoch(1758159446615);
    let id_worker1 = IdWorkerGenerator::generate(options1);
    let id_worker2 = IdWorkerGenerator::generate(options2);

    let id_workers = vec![&id_worker1, &id_worker2];

    c.bench_function("multiple_workers_normal", |b| {
        b.iter(|| {
            for id_worker in &id_workers {
                black_box(id_worker.next_id());
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
