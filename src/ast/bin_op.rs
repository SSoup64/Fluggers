use super::ast_node::AstNode;
use crate::token::Token;

#[derive(Debug)]
pub struct BinOp<'input> {
    left: Box<dyn AstNode + 'input>,
    right: Box<dyn AstNode + 'input>,
    op: Token<'input>,
}

impl<'input> BinOp<'input> {
    pub fn new(
        left: Box<dyn AstNode + 'input>,
        right: Box<dyn AstNode + 'input>,
        op: Token<'input>,
    ) -> Self {
        Self { left, right, op }
    }
}

impl<'input> AstNode for BinOp<'input> {
    fn evaluate(&self) {
        // TODO:
        // Implement
    }
}
