use crate::operator::Operator;

pub trait Operation {
    fn evaluate(&self) -> f64;
    fn operator(&self) -> Operator;
}
