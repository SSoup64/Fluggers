use super::ast_node::AstNode;

pub struct IntLiteral(i128);

impl IntLiteral {
    fn new(value: i128) -> Self {
        Self(value)
    }
}

impl AstNode for IntLiteral {
    fn evaluate(&mut self) {}
}
