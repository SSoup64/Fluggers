pub mod ast;
pub mod lexer;
pub mod parser;

use crate::lexer::LexError;
use crate::lexer::Lexer;

use crate::parser::Parser;

use std::error::Error;
use std::fs;

fn print_error(err: &LexError, code: &str) {
    match err {
        LexError::UnknownCharacter(index) => {
            println!(
                "Unknown character `{}` at index {}.",
                code.chars().nth(*index).unwrap(),
                index
            );
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let code: String = fs::read_to_string("../code.flug")?;

    let mut lexer = Lexer::new(&code);
    let lexer_result = lexer.lex();

    match lexer_result {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(errs) => {
            for err in errs {
                print_error(err, &code.to_string());
            }
        }
    }

    Ok(())
}
