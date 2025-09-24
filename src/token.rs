use crate::ast;
use crate::parser::{ Parser, BindingPower };

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'input> {
    Identifier(&'input str),
    IntLiteral(i128),
    FloatLiteral(f64),
    StringLiteral(String),
    
    Keyword(&'input str),

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

    Comma,
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

    pub fn into_nud_handler(self) -> Option<Box<dyn FnOnce(&mut Parser<'input>) -> ast::Node<'input> + 'input>> {
        match self {
            Token::IntLiteral(val) => Some(Box::new(move |_: &mut Parser| {
                ast::Node::IntLiteral(ast::IntLiteral::boxed(val))
            })),
            Token::Identifier(identifier) => Some(Box::new(move |parser: &mut Parser| {
                match parser.cur_token() {
                    Some(Token::ParenOpen) => {
                        let _ = parser.consume();

                        // Parse the arguments
                        let mut args = vec![];

                        while *parser.cur_token().expect("Got EOF while parsing a function call") != Token::ParenClose {
                            // Parse argument
                            args.push(parser.parse_expr(BindingPower::Min));

                            // Check if we should parse another argument
                            match *parser.cur_token().expect("Got EOF while parsing a function call") {
                                Token::Comma => {
                                    let _ = parser.consume();
                                    continue;
                                },
                                Token::ParenClose => break,
                                _ => { panic!("Unexpected token while parsing a function call") }
                            }
                        }
                        
                        // Parse the ParenClose token
                        let _ = parser.consume();

                        ast::Node::FuncCall(ast::FuncCall::boxed(identifier, args))
                    },
                    _ => {
                        ast::Node::NameLiteral(ast::NameLiteral::boxed(identifier))
                    }
                }
            })),
            Token::Keyword("let") => Some(Box::new(|parser: &mut Parser| {
                let Some(Token::Identifier(name)) = parser.consume() else {
                    panic!("Expected identifier");
                };
                
                _ = parser.expect_token(Token::AssignEq);

                let expr = parser.parse_expr(BindingPower::Min);

                ast::Node::VarDecl(ast::VarDecl::boxed(name, expr))
            })),
            Token::Keyword("func") => Some(Box::new(|parser: &mut Parser| {
                // Parse the parameters
                parser.expect_token(Token::ParenOpen);
                // TODO: Implement parameters
                parser.expect_token(Token::ParenClose);
                
                // Parse the body of the function
                parser.expect_token(Token::ParenCurlyOpen);
                let ast::Node::ExprList(exprs) = parser.parse_expr_list() else {
                    unreachable!();
                };
                parser.expect_token(Token::ParenCurlyClose);
                
                ast::Node::FuncDecl(ast::FuncDecl::boxed(*exprs))
            })),
            _ => None,
        }
    }
}
