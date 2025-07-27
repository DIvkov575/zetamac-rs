#[derive(Clone, PartialEq)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operation {
    fn symbol(&self) -> &str {
        match self {
            Operation::Addition => "+",
            Operation::Subtraction => "-",
            Operation::Multiplication => "*",
            Operation::Division => "/",
        }
    }

    fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..4) {
            0 => Operation::Addition,
            1 => Operation::Subtraction,
            2 => Operation::Multiplication,
            _ => Operation::Division,
        }
    }
}
