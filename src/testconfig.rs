#[derive(Clone)]
pub struct TestConfig {
    add_min_a: i32,
    add_max_a: i32,
    add_min_b: i32,
    add_max_b: i32,
    mul_min_a: i32,
    mul_max_a: i32,
    mul_min_b: i32,
    mul_max_b: i32,
    time_limit: u64,
}




impl Default for TestConfig {
    fn default() -> Self {
        Self {
            add_min_a: 2,
            add_max_a: 100,
            add_min_b: 2,
            add_max_b: 100,
            mul_min_a: 2,
            mul_max_a: 12,
            mul_min_b: 2,
            mul_max_b: 100,
            time_limit: 120,
        }
    }
}
