use crate::ast;
use crate::token::Token;

use std::collections::VecDeque;

pub struct Parser<'input> {
    tokens: VecDeque<Token<'input>>,
}

impl<'input> Parser<'input> {
    pub fn from_tokens(tokens: VecDeque<Token<'input>>) -> Self {
        Self { tokens }
    }

    pub fn into_ast(self) -> Result<Box<dyn ast::ast_node::AstNode>, ()> {
        Err(())
    }
}
