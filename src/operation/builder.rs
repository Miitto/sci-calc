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

    pub fn set_operator(&mut self, operator: Operator) -> &Self {
        if self.right.is_some() {
            if self.operator.priority() > operator.priority() {
                let op = Self::make(self.left.unwrap(), self.operator, self.right.unwrap());
            }
        }
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

        Self::make(left, self.operator, right)
    }

    pub fn make(
        left: Box<dyn Operation>,
        op: Operator,
        right: Box<dyn Operation>,
    ) -> Box<dyn Operation> {
        match op {
            Operator::Add => Box::new(Add::new(left, right)),
            Operator::Sub => Box::new(Sub::new(left, right)),
            Operator::Mul => Box::new(Mul::new(left, right)),
            Operator::Div => Box::new(Div::new(left, right)),
            _ => panic!("Invalid operator"),
        }
    }
}
