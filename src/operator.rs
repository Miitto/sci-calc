#[derive(Debug, Copy, PartialEq, Clone)]
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

    pub fn from_str(c: &str) -> Result<Self, ()> {
        Ok(match c {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            _ => return Err(()),
        })
    }

    pub fn priority(&self) -> i32 {
        match self {
            Operator::Add | Operator::Sub => 1,
            Operator::Mul | Operator::Div => 2,
            _ => 0,
        }
    }
}
