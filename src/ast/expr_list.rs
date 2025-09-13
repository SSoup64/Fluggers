use super::ast_node::AstNode;

pub struct ExprList {
    stmts: Vec<Box<dyn AstNode>>,
    tail: Box<dyn AstNode>,
}

impl ExprList {
    pub fn new(stmts: Vec<Box<dyn AstNode>>, tail: Box<dyn AstNode>) -> Self {
        Self { stmts, tail }
    }
}

impl AstNode for ExprList {
    fn evaluate(&self) {
        for stmt in &self.stmts {
            let _ = stmt.evaluate();
        }

        self.tail.evaluate()
    }
}
