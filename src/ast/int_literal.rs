#[derive(Debug)]
pub struct IntLiteral(i128);

impl IntLiteral {
    pub fn boxed(value: i128) -> Box<Self> {
        Box::new(Self(value))
    }

    pub fn evaluate(&self) {}
}
