use super::node::Node;

#[derive(Debug)]
pub struct VarDecl<'input> {
    name: &'input str,
    expr: Node<'input>,
}

impl<'input> VarDecl<'input> {
    pub fn boxed(name: &'input str, expr: Node<'input>) -> Box<Self> {
        Box::new(Self { name, expr })
    }

    pub fn evaluate(&self) {

    }
}
