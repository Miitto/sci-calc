use crate::{make_operation, operator::Operator};

use super::Operation;

impl Operation for f64 {
    fn evaluate(&self) -> f64 {
        *self
    }
    fn operator(&self) -> Operator {
        Operator::None
    }
}

make_operation!(Add, +, Add);
make_operation!(Sub, -, Sub);
make_operation!(Mul, *, Mul);
make_operation!(Div, /, Div);
