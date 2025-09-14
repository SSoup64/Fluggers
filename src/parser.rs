use crate::ast;
use crate::binding_power::BindingPower;
use crate::token::Token;

use std::collections::VecDeque;

use phf::phf_map;

pub struct Parser<'input> {
    tokens: VecDeque<Token<'input>>,
}

impl<'input> Parser<'input> {
    pub fn from_tokens(tokens: VecDeque<Token<'input>>) -> Self {
        Self { tokens }
    }

    pub fn into_ast(mut self) -> Result<ast::Node<'input>, ()> {
        Ok(self.parse_expr_list())
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
    fn parse_expr_list(&mut self) -> ast::Node<'input> {
        let mut stmts: Vec<ast::Node> = vec![];
        let tail: ast::Node;

        loop {
            let expr = self.parse_expr(BindingPower::Min);

            if let Some(Token::Seminolon) = self.cur_token() {
                stmts.push(expr);
                let _ = self.consume();
            } else {
                tail = expr;
                break;
            }
        }

        ast::Node::ExprList(ast::ExprList::boxed(stmts, tail))
    }

    // TODO: Find a way to have parse_expr private
    pub fn parse_expr(&mut self, min_bp: BindingPower) -> ast::Node<'input> {
        let cur_token = self.consume().unwrap();

        // Parse the NUD
        let nud_handler = cur_token.into_nud_handler().expect("Expected NUD handler");
        let mut left_hand_side: ast::Node = nud_handler(self);

        loop {
            // Check if we should stop the loop
            let Some(next_token) = self.cur_token() else {
                break;
            };

            let Some(bp) = next_token.get_binding_power() else {
                break;
            };

            if bp < min_bp {
                break;
            }

            // Next token
            let led_handler = self
                .consume()
                .unwrap()
                .into_led_handler()
                .expect("Expected LED handler");

            left_hand_side = led_handler(self, left_hand_side, bp);
        }

        left_hand_side
    }
}
