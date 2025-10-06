use super::node::Node;
use crate::token::Token;
use crate::gc::value::Value;

#[derive(Debug)]
pub struct BinOp<'input> {
    left: Node<'input>,
    right: Node<'input>,
    op: Token<'input>,
}

impl<'input> BinOp<'input> {
    pub fn boxed(left: Node<'input>, right: Node<'input>, op: Token<'input>) -> Box<Self> {
        Box::new(Self { left, right, op })
    }

    pub fn evaluate(&self) -> Value {
        todo!() // All the type checks.
    }
}
