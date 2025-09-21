use std::fmt;

pub struct NameLiteral<'input> {
    name: &'input str,
}

impl<'input> NameLiteral<'input> {
    pub fn boxed(name: &'input str) -> Box<Self> {
        Box::new(Self { name })
    }

    pub fn evaluate(&self) {}
}

impl<'input> fmt::Debug for NameLiteral<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NameLiteral({})", self.name)
    }
}
