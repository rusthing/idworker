#[cfg(test)]
mod generator_tests {
    use idworker::generator::Generator;
    use idworker::options::Options;

    #[test]
    fn test_next_id() {
        let options = Options::new().epoch(1).data_center(7, 3).node(1, 1);
        let mut generator = Generator::new(options);
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
        println!("id: {}", generator.next_id());
    }
}
