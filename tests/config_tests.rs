#[cfg(test)]
mod config_tests {
    use idworker::IdWorkerConfig;

    #[test]
    fn test_new_config() {
        IdWorkerConfig::builder().build().unwrap();
        IdWorkerConfig::builder()
            .epoch(1)
            .data_center(7)
            .data_center_bits(3)
            .node(1)
            .node_bits(1)
            .build()
            .expect("Fail to build");
    }

    #[test]
    #[should_panic]
    fn test_data_center_out_of_range1() {
        IdWorkerConfig::builder()
            .data_center(0)
            .data_center_bits(9)
            .build()
            .expect("Fail to set data center"); // 超出范围的数据中心ID
    }

    #[test]
    #[should_panic]
    fn test_data_center_out_of_range2() {
        IdWorkerConfig::builder()
            .data_center(8)
            .data_center_bits(3)
            .build()
            .expect("Fail to set data center"); // 超出范围的数据中心ID
    }

    #[test]
    #[should_panic]
    fn test_node_out_of_range1() {
        IdWorkerConfig::builder()
            .node(0)
            .node_bits(9)
            .build()
            .expect("Fail to set node"); // 超出范围的node ID
    }

    #[test]
    #[should_panic]
    fn test_node_out_of_range2() {
        IdWorkerConfig::builder()
            .node(1)
            .node_bits(0)
            .build()
            .expect("Fail to set node"); // 超出范围的node ID
    }
}
