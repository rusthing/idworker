// benches/fast_benches
use criterion::{criterion_group, criterion_main, Criterion};
use idworker::{IdWorkerConfig, IdWorkerGenerator, Mode};
use std::sync::{Arc, Mutex};
use std::thread;

fn bench_single_thread_id_generation(c: &mut Criterion) {
    let config = IdWorkerConfig::builder()
        .mode(Mode::Fastest)
        .epoch(1758159446615)
        .data_center(1)
        .data_center_bits(5)
        .node(1)
        .node_bits(5)
        .build()
        .expect("Failed to build id worker config");

    let id_worker = IdWorkerGenerator::generate(config).expect("Failed to generate id worker");

    c.bench_function("single_thread_id_generation", |b| {
        b.iter(|| id_worker.next_id().expect("Failed to generate id"))
    });
}

fn bench_multi_thread_id_generation(c: &mut Criterion) {
    let config = IdWorkerConfig::builder()
        .mode(Mode::Fastest)
        .epoch(1758159446615)
        .data_center(1)
        .data_center_bits(5)
        .node(1)
        .node_bits(5)
        .build()
        .expect("Failed to build id worker config");

    let generator = Arc::new(Mutex::new(
        IdWorkerGenerator::generate(config).expect("Failed to generate id worker"),
    ));

    c.bench_function("multi_thread_id_generation", |b| {
        b.iter(|| {
            let generator = Arc::clone(&generator);
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let gen_clone = Arc::clone(&generator);
                    thread::spawn(move || {
                        let g = gen_clone.lock().unwrap();
                        g.next_id().expect("Failed to generate id")
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

fn bench_id_generation_throughput(c: &mut Criterion) {
    let config = IdWorkerConfig::builder()
        .mode(Mode::Fastest)
        .epoch(1758159446615)
        .data_center(1)
        .data_center_bits(5)
        .node(1)
        .node_bits(5)
        .build()
        .expect("Failed to build id worker config");

    let generator = IdWorkerGenerator::generate(config).expect("Failed to generate id worker");

    c.bench_function("id_generation_throughput", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                generator.next_id().expect("Failed to generate id");
            }
        })
    });
}

criterion_group!(
    benches,
    bench_single_thread_id_generation,
    bench_multi_thread_id_generation,
    bench_id_generation_throughput
);

criterion_main!(benches);
