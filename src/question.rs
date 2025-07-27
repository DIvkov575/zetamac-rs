use crate::operation::Operation;
use crate::testconfig::TestConfig;
use rand::Rng;

#[derive(Clone)]
pub struct Question {
    a: i32,
    b: i32,
    operation: Operation,
    pub correct_answer: i32,
}

impl Question {
    pub fn new(config: &TestConfig) -> Self {
        let mut rng = rand::thread_rng();
        let operation = Operation::random();

        let (a, b, correct_answer) = match operation {
            Operation::Addition => {
                let a = rng.gen_range(config.add_min_a..=config.add_max_a);
                let b = rng.gen_range(config.add_min_b..=config.add_max_b);
                (a, b, a + b)
            }
            Operation::Subtraction => {
                // Use addition ranges inversely: generate result + b = a
                let result = rng.gen_range(config.add_min_a..=config.add_max_a);
                let b = rng.gen_range(config.add_min_b..=config.add_max_b);
                let a = result + b;
                (a, b, result)
            }
            Operation::Multiplication => {
                let a = rng.gen_range(config.mul_min_a..=config.mul_max_a);
                let b = rng.gen_range(config.mul_min_b..=config.mul_max_b);
                (a, b, a * b)
            }
            Operation::Division => {
                // Use multiplication ranges inversely: generate result * b = a
                let result = rng.gen_range(config.mul_min_a..=config.mul_max_a);
                let b = rng.gen_range(std::cmp::max(config.mul_min_b, 1)..=config.mul_max_b);
                let a = result * b;
                (a, b, result)
            }
        };

        Self {
            a,
            b,
            operation,
            correct_answer,
        }
    }

    pub fn display(&self) -> String {
        format!("{} {} {} = ?", self.a, self.operation.symbol(), self.b)
    }
}
