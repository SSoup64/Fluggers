pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

use crate::lexer::Lexer;
use crate::parser::Parser;

use std::process;

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let code: String = fs::read_to_string("../code.flug")?;

    let lexer = Lexer::new(&code);
    let tokens = lexer.into_tokens().unwrap_or_else(|errs| {
        for err in errs {
            println!("{}", err);
        }
        process::exit(1)
    });

    println!("Successfully parsed");

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
