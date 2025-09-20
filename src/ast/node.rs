use std::fmt;

use super::bin_op::BinOp;
use super::expr_list::ExprList;
use super::int_literal::IntLiteral;
use super::var_decl::VarDecl;

// Node enum
pub enum Node<'input> {
    BinOp(Box<BinOp<'input>>),
    ExprList(Box<ExprList<'input>>),
    IntLiteral(Box<IntLiteral>),
    VarDecl(Box<VarDecl<'input>>),
}

impl<'input> Node<'input> {
    pub fn evaluate(&self) {
        match self {
            Node::BinOp(bin_op) => bin_op.evaluate(),
            Node::ExprList(expr_list) => expr_list.evaluate(),
            Node::IntLiteral(int_literal) => int_literal.evaluate(),
            Node::VarDecl(var_decl) => var_decl.evaluate(),
        }
    }
}

impl<'input> fmt::Debug for Node<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::BinOp(bin_op) => bin_op.fmt(f),
            Node::ExprList(expr_list) => expr_list.fmt(f),
            Node::IntLiteral(int_literal) => int_literal.fmt(f),
            Node::VarDecl(var_decl) => var_decl.fmt(f),
        }
    }
}
