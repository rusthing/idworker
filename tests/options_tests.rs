#[cfg(test)]
mod options_tests {
    use idworker::options::Options;

    #[test]
    fn test_new_option() {
        Options::new();
        Options::new().epoch(1).data_center(7, 3).node(1, 1);
    }

    #[test]
    #[should_panic]
    fn test_data_center_out_of_range1() {
        Options::new().data_center(0, 9); // 超出范围的数据中心ID
    }

    #[test]
    #[should_panic]
    fn test_data_center_out_of_range2() {
        Options::new().data_center(8, 3); // 超出范围的数据中心ID
    }

    #[test]
    #[should_panic]
    fn test_node_out_of_range1() {
        Options::new().node(0, 9); // 超出范围的node ID
    }

    #[test]
    #[should_panic]
    fn test_node_out_of_range2() {
        Options::new().node(1, 0); // 超出范围的node ID
    }
}
