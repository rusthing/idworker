// benches/id_benches.rs
use criterion::{criterion_group, criterion_main, Criterion};
use idworker::generator::Generator;
use idworker::options::Options;
use std::sync::{Arc, Mutex};
use std::thread;

fn bench_single_thread_id_generation(c: &mut Criterion) {
    let options = Options::new()
        .epoch(1758159446615)
        .data_center(1, 5)
        .node(1, 5);

    let mut generator = Generator::new(options);

    c.bench_function("single_thread_id_generation", |b| {
        b.iter(|| generator.next_id())
    });
}

fn bench_multi_thread_id_generation(c: &mut Criterion) {
    let options = Options::new()
        .epoch(1758159446615)
        .data_center(1, 5)
        .node(1, 5);

    let generator = Arc::new(Mutex::new(Generator::new(options)));

    c.bench_function("multi_thread_id_generation", |b| {
        b.iter(|| {
            let generator = Arc::clone(&generator);
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let gen_clone = Arc::clone(&generator);
                    thread::spawn(move || {
                        let mut g = gen_clone.lock().unwrap();
                        g.next_id()
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
    let options = Options::new()
        .epoch(1758159446615)
        .data_center(1, 5)
        .node(1, 5);

    let mut generator = Generator::new(options);

    c.bench_function("id_generation_throughput", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                generator.next_id();
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
