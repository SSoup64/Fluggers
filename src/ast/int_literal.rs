use super::ast_node::AstNode;

#[derive(Debug)]
pub struct IntLiteral(i128);

impl IntLiteral {
    pub fn new(value: i128) -> Self {
        Self(value)
    }
}

impl AstNode for IntLiteral {
    fn evaluate(&self) {}
}
