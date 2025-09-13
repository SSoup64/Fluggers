use crate::ast::{self, IntLiteral};
use crate::ast::ast_node::AstNode;

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
    fn parse_expr_list(&mut self) -> ast::ExprList {
        let mut stmts: Vec<Box<dyn AstNode>> = vec![];
        let tail: Box<dyn AstNode>;

        loop {
            let expr = self.parse_expr();
            
            if let Some(Token::Seminolon) = self.cur_token() {
                stmts.push(expr);
                let _ = self.consume();
            } else {
                tail = expr;
                break;
            }
        }

        ast::ExprList::new(stmts, tail)
    }

    fn parse_expr(&mut self) -> Box<dyn AstNode> {
        // Placeholder implementation
        Box::new(ast::IntLiteral::new(1))
    }
}










