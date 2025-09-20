use std::fmt;

pub struct IntLiteral(i128);

impl IntLiteral {
    pub fn boxed(value: i128) -> Box<Self> {
        Box::new(Self(value))
    }

    pub fn evaluate(&self) {}
}

impl fmt::Debug for IntLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IntLiteral({})", self.0)
    }
}
