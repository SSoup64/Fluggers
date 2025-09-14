use super::bin_op::BinOp;
use super::expr_list::ExprList;
use super::int_literal::IntLiteral;

// Node enum
#[derive(Debug)]
pub enum Node<'input> {
    BinOp(Box<BinOp<'input>>),
    ExprList(Box<ExprList<'input>>),
    IntLiteral(Box<IntLiteral>),
}

impl<'input> Node<'input> {
    pub fn evaluate(&self) {
        match self {
            Node::BinOp(bin_op) => bin_op.evaluate(),
            Node::ExprList(expr_list) => expr_list.evaluate(),
            Node::IntLiteral(int_literal) => int_literal.evaluate(),
        }
    }
}



