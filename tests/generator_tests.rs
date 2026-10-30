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

    /// 回归：序列号用尽时早期实现会直接回绕，同一时间刻度内二次发号 → 重复 ID。
    ///
    /// `sequence_bits = 4` 让每个刻度只有 16 个号，单线程连发几万条必然打满，
    /// 修复后应「等到下一个刻度」而不是回绕。
    #[test]
    fn test_normal_sequence_exhausted_no_duplicate() {
        let config = IdWorkerConfig::builder()
            .mode(Mode::Normal)
            .epoch(1)
            .node(0)
            .sequence_bits(4)
            .build()
            .expect("Failed to build id worker config");
        let id_worker = IdWorkerGenerator::generate(config).expect("Failed to generate id worker");

        let ids: Vec<u64> = (0..50_000)
            .map(|_| id_worker.next_id().expect("Failed to generate id"))
            .collect();
        let unique: std::collections::HashSet<u64> = ids.iter().copied().collect();
        assert_eq!(unique.len(), ids.len(), "序列号用尽后出现了重复 ID");
        assert!(
            ids.windows(2).all(|w| w[0] < w[1]),
            "Normal 模式必须严格递增"
        );
    }

    /// 回归：`Faster` / `Fastest` 在序列号用尽处曾经生成重复、甚至倒退的 ID。
    #[test]
    fn test_fast_modes_sequence_exhausted_no_duplicate() {
        for mode in [Mode::Faster, Mode::Fastest] {
            let config = IdWorkerConfig::builder()
                .mode(mode.clone())
                .epoch(1)
                .node(0)
                .node_bits(1)
                .sequence_bits(1)
                .build()
                .expect("Failed to build id worker config");
            let id_worker =
                IdWorkerGenerator::generate(config).expect("Failed to generate id worker");

            let ids: Vec<u64> = (0..2_000)
                .map(|_| id_worker.next_id().expect("Failed to generate id"))
                .collect();
            let unique: std::collections::HashSet<u64> = ids.iter().copied().collect();
            assert_eq!(unique.len(), ids.len(), "{mode:?} 出现了重复 ID");
            assert!(ids.windows(2).all(|w| w[0] < w[1]), "{mode:?} 必须严格递增");
        }
    }

    /// 各精度档位都要能正常发号且不重复（epoch 按同量纲传入）。
    #[test]
    fn test_epoch_precision_levels() {
        for (precision, epoch) in [(1i8, 1u64), (0, 1), (-1, 1), (-2, 1)] {
            let config = IdWorkerConfig::builder()
                .mode(Mode::Normal)
                .epoch(epoch)
                .epoch_precision(precision)
                .node(0)
                .build()
                .expect("Failed to build id worker config");
            let id_worker =
                IdWorkerGenerator::generate(config).expect("Failed to generate id worker");

            let ids: Vec<u64> = (0..5_000)
                .map(|_| id_worker.next_id().expect("Failed to generate id"))
                .collect();
            let unique: std::collections::HashSet<u64> = ids.iter().copied().collect();
            assert_eq!(unique.len(), ids.len(), "precision={precision} 出现重复 ID");
        }
    }

    /// precision 越界、以及位宽越过 i64 的配置必须被拦下，而不是发出负数 ID。
    #[test]
    fn test_invalid_precision_rejected() {
        for precision in [4i8, -4] {
            let r = IdWorkerConfig::builder().epoch_precision(precision).build();
            assert!(r.is_err(), "precision={precision} 应被拒绝");
        }
        // precision = 2（0.01ms）会把时间戳推到 48 位，叠加默认 16 位后越过 63
        assert!(
            IdWorkerConfig::builder()
                .epoch_precision(2)
                .build()
                .is_err(),
            "位宽超 i64 的配置应被拒绝"
        );
    }
}
