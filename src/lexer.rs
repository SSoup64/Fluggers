use std::collections::VecDeque;
use std::fmt;

use phf::{ phf_set, phf_map };

use crate::token::Token;

const KEYWORDS: phf::Set<&'static str> = phf_set! {
    "func",

    "let",
    "const",

    "if",
    "elif",
    "else",

    "while",

    "true",
    "false",
};

// In order to make stuff like += easier to implement down the line
const OPERATORS: phf::Map<char, Token> = phf_map! {
    '+' => Token::Plus,
    '-' => Token::Minus,
    '*' => Token::Star,
    '/' => Token::Slash,
};

#[derive(Debug)]
pub enum LexErrorType {
    UnknownCharacter(usize),
}

pub struct LexError<'input> {
    input: &'input String,
    error: LexErrorType,
}

impl<'input> LexError<'input> {
    fn new(input: &'input String, error: LexErrorType) -> Self {
        Self { input, error }
    }
}

impl fmt::Display for LexError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error {
            LexErrorType::UnknownCharacter(pos) => {
                write!(
                    f,
                    "Unknown character `{}` at position {}",
                    self.input.chars().nth(pos).unwrap(),
                    pos
                )
            }
        }
    }
}

pub struct Lexer<'input> {
    input: &'input String,
    index: usize,
    tokens: VecDeque<Token<'input>>,
    errs: VecDeque<LexError<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input String) -> Self {
        Self {
            input,
            index: 0,
            tokens: VecDeque::<Token>::new(),
            errs: VecDeque::<LexError>::new(),
        }
    }

    fn get_cur_char(&self) -> Option<char> {
        self.input.chars().nth(self.index)
    }

    fn get_next_char(&self) -> Option<char> {
        self.input.chars().nth(self.index + 1)
    }

    pub fn into_tokens(mut self) -> Result<VecDeque<Token<'input>>, VecDeque<LexError<'input>>> {
        // TODO: Implement states for lexer so we don't have to reset the stuff
        self.index = 0;
        self.tokens.clear();
        self.errs.clear();

        'input_loop: while let Some(ch) = self.get_cur_char() {
            if ch.is_whitespace() {
                self.index += 1;
                continue 'input_loop;
            }

            match ch {
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.lex_identifier();
                }
                '0'..='9' => {
                    self.lex_number();
                }

                '+' | '-' | '*' | '/' => {
                    if let Some(operator) = OPERATORS.get(&ch) {
                        // TODO: Check if next char is =
                        self.tokens.push_back(operator.clone());
                    } else {
                        unreachable!();
                    }
                }

                '<' => {
                    if let Some('=') = self.get_next_char() {
                        self.tokens.push_back(Token::LesserEq);
                        self.index += 1;
                    } else {
                        self.tokens.push_back(Token::Lesser);
                    }
                }

                '>' => {
                    if let Some('=') = self.get_next_char() {
                        self.tokens.push_back(Token::GreaterEq);
                        self.index += 1;
                    } else {
                        self.tokens.push_back(Token::Greater);
                    }
                }

                '=' => {
                    if let Some('=') = self.get_next_char() {
                        self.tokens.push_back(Token::IsEq);
                        self.index += 1; // For the second =
                    } else {
                        self.tokens.push_back(Token::AssignEq);
                    }
                }

                '(' => {
                    self.tokens.push_back(Token::ParenOpen);
                }
                ')' => {
                    self.tokens.push_back(Token::ParenClose);
                }

                '{' => {
                    self.tokens.push_back(Token::ParenCurlyOpen);
                }
                '}' => {
                    self.tokens.push_back(Token::ParenCurlyClose);
                }

                ';' => {
                    self.tokens.push_back(Token::Semicolon);
                }

                ',' => {
                    self.tokens.push_back(Token::Comma);
                }

                _ => {
                    self.errs.push_back(LexError::new(
                        self.input,
                        LexErrorType::UnknownCharacter(self.index),
                    ));
                }
            }

            self.index += 1;
        }

        if self.errs.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.errs)
        }
    }

    fn lex_identifier(&mut self) {
        let start = self.index;

        while let Some(ch) = self.get_cur_char() {
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    self.index += 1;
                }
                _ => {
                    break;
                }
            }
        }
        
        self.index -= 1;

        let identifier = &self.input[start..=self.index];

        if KEYWORDS.contains(identifier) {
            self.tokens.push_back(Token::Keyword(identifier));
        } else {
            self.tokens.push_back(Token::Identifier(identifier));
        }
    }

    fn lex_number(&mut self) {
        let start = self.index;
        let mut found_dot = false;

        while let Some(ch) = self.get_cur_char() {
            match ch {
                '0'..='9' | '_' => {
                    self.index += 1;
                }
                '.' => {
                    if found_dot {
                        // Report an error
                        // Wait until next symnol that is not a digit, or a dot and resume parsing.
                    } else {
                        self.index += 1;
                        found_dot = true;
                    }
                }
                _ => {
                    break;
                }
            }
        }
        
        self.index -= 1;

        let number_str = &self.input[start..=self.index];

        if found_dot {
            match number_str.parse::<f64>() {
                Ok(number) => {
                    self.tokens.push_back(Token::FloatLiteral(number));
                }
                Err(_) => {
                    unreachable!();
                }
            }
        } else {
            match number_str.parse::<i128>() {
                Ok(number) => {
                    self.tokens.push_back(Token::IntLiteral(number));
                }
                Err(_) => {
                    unreachable!();
                }
            }
        }
    }
}
