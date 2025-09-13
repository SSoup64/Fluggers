use super::ast_node::AstNode;

#[derive(Debug)]
pub struct ExprList<'input> {
    stmts: Vec<Box<dyn AstNode + 'input>>,
    tail: Box<dyn AstNode + 'input>,
}

impl<'input> ExprList<'input> {
    pub fn new(stmts: Vec<Box<dyn AstNode + 'input>>, tail: Box<dyn AstNode + 'input>) -> Self {
        Self { stmts, tail }
    }
}

impl<'input> AstNode for ExprList<'input> {
    fn evaluate(&self) {
        for stmt in &self.stmts {
            let _ = stmt.evaluate();
        }

        self.tail.evaluate()
    }
}
