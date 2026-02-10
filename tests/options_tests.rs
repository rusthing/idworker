#[cfg(test)]
mod options_tests {
    use idworker::IdWorkerOptions;

    #[test]
    fn test_new_option() {
        IdWorkerOptions::new();
        IdWorkerOptions::new()
            .epoch(1)
            .expect("Fail to set epoch")
            .data_center(7, 3)
            .expect("Fail to set data center")
            .node(1, 1)
            .expect("Fail to set node");
    }

    #[test]
    #[should_panic]
    fn test_data_center_out_of_range1() {
        IdWorkerOptions::new()
            .data_center(0, 9)
            .expect("Fail to set data center"); // 超出范围的数据中心ID
    }

    #[test]
    #[should_panic]
    fn test_data_center_out_of_range2() {
        IdWorkerOptions::new()
            .data_center(8, 3)
            .expect("Fail to set data center"); // 超出范围的数据中心ID
    }

    #[test]
    #[should_panic]
    fn test_node_out_of_range1() {
        IdWorkerOptions::new().node(0, 9).expect("Fail to set node"); // 超出范围的node ID
    }

    #[test]
    #[should_panic]
    fn test_node_out_of_range2() {
        IdWorkerOptions::new().node(1, 0).expect("Fail to set node"); // 超出范围的node ID
    }
}
