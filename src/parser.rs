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

    pub fn into_ast(mut self) -> Result<Box<dyn ast::ast_node::AstNode>, ()> {
        let _ = self.parse_expr_list();
        Err(())
    }

    // Util funcs
    fn cur_token(&self) -> Option<&Token<'input>> {
        self.tokens.front()
    }

    fn peek(&self, index: usize) -> Option<&Token<'input>> {
        self.tokens.get(index)
    }

    fn consume(&mut self) -> Option<Token<'input>> {
        self.tokens.pop_front()
    }

    // Parsing funcs
    fn parse_expr_list(&mut self) {
        loop {
            // TODO: Parse expr
            
            if let Some(Token::Seminolon) = self.cur_token() {
                let _ = self.consume();
            } else {
                break;
            }
        }
    }
}
