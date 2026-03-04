#[cfg(test)]
mod generator_tests {
    use idworker::{IdWorkerConfig, IdWorkerGenerator, Mode};
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_normal() {
        let config = IdWorkerConfig::builder()
            .mode(Mode::Normal)
            .epoch(1)
            .data_center(7)
            .data_center_bits(3)
            .node(1)
            .node_bits(1)
            .build()
            .expect("Failed to build id worker config");

        let id_worker = IdWorkerGenerator::generate(config).expect("Failed to generate id worker");
        for _ in 0..1000 {
            println!(
                "id: {}",
                id_worker.next_id().expect("Failed to generate id")
            );
        }
    }

    #[test]
    fn test_faster() {
        let config = IdWorkerConfig::builder()
            .mode(Mode::Faster)
            .epoch(1)
            .data_center(7)
            .data_center_bits(3)
            .node(1)
            .node_bits(1)
            .build()
            .expect("Failed to build id worker config");

        let id_worker = IdWorkerGenerator::generate(config).expect("Failed to generate id worker");
        for _ in 0..1000 {
            println!(
                "id: {}",
                id_worker.next_id().expect("Failed to generate id")
            );
        }
    }

    #[test]
    fn test_fastest() {
        let config = IdWorkerConfig::builder()
            .mode(Mode::Fastest)
            .epoch(1)
            .data_center(7)
            .data_center_bits(3)
            .node(1)
            .node_bits(1)
            .build()
            .expect("Failed to build id worker config");

        let id_worker = IdWorkerGenerator::generate(config).expect("Failed to generate id worker");
        for _ in 0..1000 {
            println!(
                "id: {}",
                id_worker.next_id().expect("Failed to generate id")
            );
        }
    }

    #[test]
    fn test_normal_multithread() {
        let config = IdWorkerConfig::builder()
            .mode(Mode::Normal)
            .epoch(1)
            .data_center(7)
            .data_center_bits(3)
            .node(1)
            .node_bits(1)
            .build()
            .expect("Failed to build id worker config");

        let id_worker =
            Arc::new(IdWorkerGenerator::generate(config).expect("Failed to generate id worker"));
        let mut handles = vec![];

        for _ in 0..10 {
            let worker = Arc::clone(&id_worker);
            let handle = thread::spawn(move || {
                let mut ids = Vec::new();
                for _ in 0..100 {
                    ids.push(worker.next_id().expect("Failed to generate id"));
                }
                ids
            });
            handles.push(handle);
        }

        let mut all_ids = Vec::new();
        for handle in handles {
            let ids = handle.join().unwrap();
            all_ids.extend(ids);
        }

        // 验证生成的 ID 数量
        assert_eq!(all_ids.len(), 1000);

        // 验证 ID 的唯一性
        let unique_ids: std::collections::HashSet<u64> = all_ids.into_iter().collect();
        assert_eq!(unique_ids.len(), 1000);
    }

    #[test]
    fn test_faster_multithread() {
        let config = IdWorkerConfig::builder()
            .mode(Mode::Faster)
            .epoch(1)
            .data_center(7)
            .data_center_bits(3)
            .node(1)
            .node_bits(1)
            .build()
            .expect("Failed to build id worker config");

        let id_worker =
            Arc::new(IdWorkerGenerator::generate(config).expect("Failed to generate id worker"));
        let mut handles = vec![];

        for _ in 0..10 {
            let worker = Arc::clone(&id_worker);
            let handle = thread::spawn(move || {
                let mut ids = Vec::new();
                for _ in 0..100 {
                    ids.push(worker.next_id().expect("Failed to generate id"));
                }
                ids
            });
            handles.push(handle);
        }

        let mut all_ids = Vec::new();
        for handle in handles {
            let ids = handle.join().unwrap();
            all_ids.extend(ids);
        }

        // 验证生成的 ID 数量
        assert_eq!(all_ids.len(), 1000);

        // 验证 ID 的唯一性
        let unique_ids: std::collections::HashSet<u64> = all_ids.into_iter().collect();
        assert_eq!(unique_ids.len(), 1000);
    }

    #[test]
    fn test_fastest_multithread() {
        let config = IdWorkerConfig::builder()
            .mode(Mode::Fastest)
            .epoch(1)
            .data_center(7)
            .data_center_bits(3)
            .node(1)
            .node_bits(1)
            .build()
            .expect("Failed to build id worker config");

        let id_worker =
            Arc::new(IdWorkerGenerator::generate(config).expect("Failed to generate id worker"));
        let mut handles = vec![];

        for _ in 0..10 {
            let worker = Arc::clone(&id_worker);
            let handle = thread::spawn(move || {
                let mut ids = Vec::new();
                for _ in 0..100 {
                    ids.push(worker.next_id().expect("Failed to generate id"));
                }
                ids
            });
            handles.push(handle);
        }

        let mut all_ids = Vec::new();
        for handle in handles {
            let ids = handle.join().unwrap();
            all_ids.extend(ids);
        }

        // 验证生成的 ID 数量
        assert_eq!(all_ids.len(), 1000);

        // 验证 ID 的唯一性
        let unique_ids: std::collections::HashSet<u64> = all_ids.into_iter().collect();
        assert_eq!(unique_ids.len(), 1000);
    }
}
