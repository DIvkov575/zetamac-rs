use rand::Rng;

#[derive(Clone, PartialEq)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operation {
    pub fn symbol(&self) -> &str {
        match self {
            Operation::Addition => "+",
            Operation::Subtraction => "-",
            Operation::Multiplication => "*",
            Operation::Division => "/",
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Operation::Addition,
            1 => Operation::Subtraction,
            2 => Operation::Multiplication,
            _ => Operation::Division,
        }
    }
}
