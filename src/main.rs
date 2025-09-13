pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

use crate::lexer::Lexer;
use crate::parser::Parser;

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let code: String = fs::read_to_string("../code.flug")?;

    // Lexer
    let lexer = Lexer::new(&code);
    let tokens = lexer.into_tokens().map_err(|errs| {
        for err in errs {
            eprintln!("{}", err);
        }
        "Failed at lexing"
    })?;

    // Parser
    let parser = Parser::from_tokens(tokens);
    let _ast = parser.into_ast();

    Ok(())
}
