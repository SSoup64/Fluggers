use crate::ast;
use crate::binding_power::BindingPower;
use crate::parser::Parser;

#[derive(Debug, Clone, PartialEq)]
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

    Semicolon,
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
            dyn FnOnce(&mut Parser<'input>, ast::Node<'input>, BindingPower) -> ast::Node<'input>
                + 'input,
        >,
    > {
        match self {
            op @ (Token::Plus | Token::Minus | Token::Star | Token::Slash) => {
                Some(Box::new(move |parser, left, bp| {
                    let right = parser.parse_expr(bp);
                    ast::Node::BinOp(ast::BinOp::boxed(left, right, op))
                }))
            }
            _ => None,
        }
    }

    pub fn into_nud_handler(self) -> Option<Box<dyn FnOnce(&mut Parser) -> ast::Node<'input>>> {
        match self {
            Token::IntLiteral(val) => Some(Box::new(move |_: &mut Parser| {
                ast::Node::IntLiteral(ast::IntLiteral::boxed(val))
            })),
            _ => None,
        }
    }
}
