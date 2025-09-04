pub mod lexer;
pub mod parser;
pub mod ast;

use crate::lexer::Lexer;
use crate::lexer::LexError;

use crate::parser::Parser;

use std::fs;
use std::error::Error;

fn print_error(err: &LexError, code: &String) {
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
        },
        Err(errs) => {
            for err in errs {
                print_error(&err, &code);
            }
        }
    }

    Ok(())
}





