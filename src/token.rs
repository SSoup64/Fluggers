use crate::ast::{ast_node::AstNode, *};
use crate::binding_power::BindingPower;
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub enum Token<'input> {
    Identifier(&'input str),
    IntLiteral(i128),
    FloatLiteral(f64),
    StringLiteral(String),

    // Keywords
    KeywordFunc,

    KeywordLet,
    KeywordConst,

    KeywordIf,
    KeywordElif,
    KeywordElse,

    KeywordWhile,

    KeywordTrue,
    KeywordFalse,

    // Symbols
    Greater,
    Lesser,

    IsEq,

    GreaterEq,
    LesserEq,

    SymbolArrow,

    AssignEq,

    Plus,
    Minus,

    Star,
    Slash,

    ParenOpen,
    ParenClose,

    ParenCurlyOpen,
    ParenCurlyClose,

    Seminolon,
}

impl<'input> Token<'input> {
    pub fn get_binding_power(&self) -> Option<BindingPower> {
        match self {
            Token::Plus | Token::Minus => Some(BindingPower::Additive),
            Token::Star | Token::Slash => Some(BindingPower::Multiplicative),
            Token::ParenOpen => Some(BindingPower::Group),
            _ => None,
        }
    }

    pub fn into_led_handler(
        self,
    ) -> Option<
        Box<
            dyn FnOnce(
                    &mut Parser<'input>,
                    Box<dyn AstNode + 'input>,
                    BindingPower,
                ) -> Box<dyn AstNode + 'input>
                + 'input,
        >,
    > {
        match self {
            op @ (Token::Plus | Token::Minus | Token::Star | Token::Slash) => Some(Box::new(move |parser, left, bp| {
                let right = parser.parse_expr(bp);
                Box::new(BinOp::new(left, right, op))
            })),
            _ => None,
        }
    }

    pub fn into_nud_handler(self) -> Option<Box<dyn FnOnce(&mut Parser) -> Box<dyn AstNode>>> {
        match self {
            Token::IntLiteral(val) => Some(Box::new(move |_: &mut Parser| {
                Box::new(IntLiteral::new(val))
            })),
            _ => None,
        }
    }
}
