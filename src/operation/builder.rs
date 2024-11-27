use crate::operator::Operator;

use super::{base::*, Operation};

pub struct Builder {
    left: Option<Box<dyn Operation>>,
    right: Option<Box<dyn Operation>>,
    operator: Operator,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
            operator: Operator::None,
        }
    }
    pub fn set_left(&mut self, left: Box<dyn Operation>) -> &Self {
        self.left = Some(left);
        self
    }
    pub fn set_right(&mut self, right: Box<dyn Operation>) -> &Self {
        self.right = Some(right);
        self
    }
    pub fn operator(&mut self, operator: Operator) -> &Self {
        self.operator = operator;
        self
    }

    pub fn next(&mut self, next: Box<dyn Operation>) -> &Self {
        if self.left.is_none() {
            self.left = Some(next);
        } else if self.right.is_none() {
            self.right = Some(next);
        } else {
            panic!("Both left and right are already set");
        }
        self
    }

    pub fn from_str<'a>(&mut self, token: &'a str) -> &Self {
        if Operator::is_operator(token) {
            self.operator(Operator::from_str(token));
        } else {
            let val_r = token.parse::<f64>();
            if val_r.is_err() {
                panic!("Invalid token: {}", token);
            }

            let val = val_r.unwrap();

            let boxed = Box::new(val);

            self.next(boxed);
        }

        self
    }

    pub fn build(self) -> Box<dyn Operation> {
        if self.left.is_none() {
            panic!("Left is not set");
        }

        let left = self.left.unwrap();

        if self.operator == Operator::None {
            return left;
        }

        if self.operator != Operator::None && self.right.is_none() {
            panic!("Right is not set and has operator");
        }

        let right = self.right.unwrap();

        match self.operator {
            Operator::Add => Box::new(Add::new(left, right)),
            Operator::Sub => Box::new(Sub::new(left, right)),
            Operator::Mul => Box::new(Mul::new(left, right)),
            Operator::Div => Box::new(Div::new(left, right)),
            _ => panic!("Invalid operator"),
        }
    }
}
