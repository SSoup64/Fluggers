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

    let lexer = Lexer::new(&code);
    let lexer_result = lexer.into_tokens();

    match lexer_result {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(errs) => {
            for err in errs {
                println!("{}", err);
            }
        }
    }

    Ok(())
}
