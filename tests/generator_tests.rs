#[cfg(test)]
mod generator_tests {
    use idworker::generator::IdWorkerGenerator;
    use idworker::options::{Mode, Options};

    #[test]
    fn test_faster() {
        let options = Options::new()
            .mode(Mode::Faster)
            .epoch(1)
            .data_center(7, 3)
            .node(1, 1);

        let id_worker = IdWorkerGenerator::generate(options);
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
    }
    #[test]
    fn test_fastest() {
        let options = Options::new()
            .mode(Mode::Fastest)
            .epoch(1)
            .data_center(7, 3)
            .node(1, 1);

        let id_worker = IdWorkerGenerator::generate(options);
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
        println!("id: {}", id_worker.next_id());
    }
}
