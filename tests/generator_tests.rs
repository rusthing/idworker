#[cfg(test)]
mod generator_tests {
    use idworker::{IdWorkerGenerator, Mode, Options};
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_normal() {
        let options = Options::new()
            .mode(Mode::Normal)
            .epoch(1)
            .data_center(7, 3)
            .node(1, 1);

        let id_worker = IdWorkerGenerator::generate(options);
        for _ in 0..1000 {
            println!("id: {}", id_worker.next_id());
        }
    }

    #[test]
    fn test_faster() {
        let options = Options::new()
            .mode(Mode::Faster)
            .epoch(1)
            .data_center(7, 3)
            .node(1, 1);

        let id_worker = IdWorkerGenerator::generate(options);
        for _ in 0..1000 {
            println!("id: {}", id_worker.next_id());
        }
    }

    #[test]
    fn test_fastest() {
        let options = Options::new()
            .mode(Mode::Fastest)
            .epoch(1)
            .data_center(7, 3)
            .node(1, 1);

        let id_worker = IdWorkerGenerator::generate(options);
        for _ in 0..1000 {
            println!("id: {}", id_worker.next_id());
        }
    }

    #[test]
    fn test_normal_multithread() {
        let options = Options::new()
            .mode(Mode::Normal)
            .epoch(1)
            .data_center(7, 3)
            .node(1, 1);

        let id_worker = Arc::new(IdWorkerGenerator::generate(options));
        let mut handles = vec![];

        for _ in 0..10 {
            let worker = Arc::clone(&id_worker);
            let handle = thread::spawn(move || {
                let mut ids = Vec::new();
                for _ in 0..100 {
                    ids.push(worker.next_id());
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
        let options = Options::new()
            .mode(Mode::Faster)
            .epoch(1)
            .data_center(7, 3)
            .node(1, 1);

        let id_worker = Arc::new(IdWorkerGenerator::generate(options));
        let mut handles = vec![];

        for _ in 0..10 {
            let worker = Arc::clone(&id_worker);
            let handle = thread::spawn(move || {
                let mut ids = Vec::new();
                for _ in 0..100 {
                    ids.push(worker.next_id());
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
        let options = Options::new()
            .mode(Mode::Fastest)
            .epoch(1)
            .data_center(7, 3)
            .node(1, 1);

        let id_worker = Arc::new(IdWorkerGenerator::generate(options));
        let mut handles = vec![];

        for _ in 0..10 {
            let worker = Arc::clone(&id_worker);
            let handle = thread::spawn(move || {
                let mut ids = Vec::new();
                for _ in 0..100 {
                    ids.push(worker.next_id());
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
