#[macro_export]
macro_rules! make_operation {
    ($name:ident, $symbol:tt, $op:ident) => {
        pub struct $name {
            left: Box<dyn $crate::operation::Operation>,
            right: Box<dyn $crate::operation::Operation>,
        }

        impl $name {
            pub fn new(left: Box<dyn $crate::operation::Operation>, right: Box<dyn $crate::operation::Operation>) -> Self {
                Self { left, right }
            }
        }

        impl $crate::operation::Operation for $name {
            fn evaluate(&self) -> f64 {
                self.left.evaluate() $symbol self.right.evaluate()
            }
            fn operator(&self) -> $crate::operator::Operator {
                $crate::operator::Operator::$op
            }
        }
    };
}
