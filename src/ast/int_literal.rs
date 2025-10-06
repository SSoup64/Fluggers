use std::fmt;
use crate::gc::value::Value;

pub struct IntLiteral(i128);

impl IntLiteral {
    pub fn boxed(value: i128) -> Box<Self> {
        Box::new(Self(value))
    }

    pub fn evaluate(&self) -> Value {
        Value::Int(self.0)
    }
}

impl fmt::Debug for IntLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IntLiteral({})", self.0)
    }
}
