use super::node::Node;
use crate::gc::value::Value;

#[derive(Debug)]
pub struct ExprList<'input> {
    stmts: Vec<Node<'input>>,
    tail: Node<'input>,
}

impl<'input> ExprList<'input> {
    pub fn boxed(stmts: Vec<Node<'input>>, tail: Node<'input>) -> Box<Self> {
        Box::new(Self { stmts, tail })
    }

    pub fn evaluate(&self) -> Value {
        for stmt in &self.stmts {
            let _ = stmt.evaluate();
        }

        self.tail.evaluate()
    }
}
