use std::fmt;

use crate::gc::value::Value;

use super::int_literal::IntLiteral;
use super::name_literal::NameLiteral;
use super::bin_op::BinOp;
use super::expr_list::ExprList;
use super::var_decl::VarDecl;
use super::func_decl::FuncDecl;
use super::func_call::FuncCall;

// Node enum
pub enum Node<'input> {
    IntLiteral(Box<IntLiteral>),
    NameLiteral(Box<NameLiteral<'input>>),
    BinOp(Box<BinOp<'input>>),
    ExprList(Box<ExprList<'input>>),
    VarDecl(Box<VarDecl<'input>>),
    FuncDecl(Box<FuncDecl<'input>>),
    FuncCall(Box<FuncCall<'input>>),
}

impl<'input> Node<'input> {
    pub fn evaluate(&self) -> Value {
        match self {
            Node::IntLiteral(int_literal) => int_literal.evaluate(),
            Node::NameLiteral(name_literal) => name_literal.evaluate(),
            Node::BinOp(bin_op) => bin_op.evaluate(),
            Node::ExprList(expr_list) => expr_list.evaluate(),
            Node::VarDecl(var_decl) => var_decl.evaluate(),
            Node::FuncDecl(func_decl) => func_decl.evaluate(),
            Node::FuncCall(func_call) => func_call.evaluate(),
        }
    }
}

impl<'input> fmt::Debug for Node<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::IntLiteral(int_literal) => int_literal.fmt(f),
            Node::NameLiteral(name_literal) => name_literal.fmt(f),
            Node::BinOp(bin_op) => bin_op.fmt(f),
            Node::ExprList(expr_list) => expr_list.fmt(f),
            Node::VarDecl(var_decl) => var_decl.fmt(f),
            Node::FuncDecl(func_decl) => func_decl.fmt(f),
            Node::FuncCall(func_call) => func_call.fmt(f),
        }
    }
}
