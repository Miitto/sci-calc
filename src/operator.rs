pub enum Operator {
    None,
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn as_str(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            _ => "",
        }
    }

    pub fn is_operator(c: &str) -> bool {
        match c {
            "+" | "-" | "*" | "/" => true,
            _ => false,
        }
    }

    pub fn from_str(c: &str) -> Self {
        match c {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            _ => Operator::None,
        }
    }

    pub fn priority(&self) -> i32 {
        match self {
            Operator::Add | Operator::Sub => 1,
            Operator::Mul | Operator::Div => 2,
            _ => 0,
        }
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Operator::Add, Operator::Add)
            | (Operator::Sub, Operator::Sub)
            | (Operator::Mul, Operator::Mul)
            | (Operator::Div, Operator::Div) => true,
            _ => false,
        }
    }
}
