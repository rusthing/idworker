// benches/fast_benches
use criterion::{criterion_group, criterion_main, Criterion};
use idworker::{IdWorkerGenerator, IdWorkerOptions, Mode};
use std::sync::{Arc, Mutex};
use std::thread;

fn bench_single_thread_id_generation(c: &mut Criterion) {
    let options = IdWorkerOptions::new()
        .mode(Mode::Fastest)
        .epoch(1758159446615)
        .expect("Failed to set epoch")
        .data_center(1, 5)
        .expect("Failed to set data center")
        .node(1, 5)
        .expect("Failed to set node");

    let id_worker = IdWorkerGenerator::generate(options).expect("Failed to generate id worker");

    c.bench_function("single_thread_id_generation", |b| {
        b.iter(|| id_worker.next_id().expect("Failed to generate id"))
    });
}

fn bench_multi_thread_id_generation(c: &mut Criterion) {
    let options = IdWorkerOptions::new()
        .mode(Mode::Fastest)
        .epoch(1758159446615)
        .expect("Failed to set epoch")
        .data_center(1, 5)
        .expect("Failed to set data center")
        .node(1, 5)
        .expect("Failed to set node");

    let generator = Arc::new(Mutex::new(
        IdWorkerGenerator::generate(options).expect("Failed to generate id worker"),
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
    let options = IdWorkerOptions::new()
        .mode(Mode::Fastest)
        .epoch(1758159446615)
        .expect("Failed to set epoch")
        .data_center(1, 5)
        .expect("Failed to set data center")
        .node(1, 5)
        .expect("Failed to set node");

    let generator = IdWorkerGenerator::generate(options).expect("Failed to generate id worker");

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
