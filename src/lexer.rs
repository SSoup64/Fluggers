use std::fmt;

use phf::phf_map;

const KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "func" => Token::KeywordFunc,

    "let" => Token::KeywordLet,
    "const" => Token::KeywordConst,

    "if" => Token::KeywordIf,
    "elif" => Token::KeywordElif,
    "else" => Token::KeywordElse,

    "while" => Token::KeywordWhile,

    "true" => Token::KeywordTrue,
    "false" => Token::KeywordFalse,
};

// In order to make stuff like += easier to implement down the line
const OPERATORS: phf::Map<char, Token> = phf_map! {
    '+' => Token::Plus,
    '-' => Token::Minus,
    '*' => Token::Star,
    '/' => Token::Slash,
};

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

pub struct Lexer<'lexer> {
    input: &'lexer String,
    index: usize,
    tokens: Vec<Token<'lexer>>,
    errs: Vec<LexError<'lexer>>,
}

impl<'lexer> Lexer<'lexer> {
    pub fn new(input: &'lexer String) -> Self {
        Self {
            input,
            index: 0,
            tokens: Vec::<Token>::new(),
            errs: Vec::<LexError>::new(),
        }
    }

    fn get_cur_char(&self) -> Option<char> {
        self.input.chars().nth(self.index)
    }

    fn get_next_char(&self) -> Option<char> {
        self.input.chars().nth(self.index + 1)
    }

    pub fn lex(
        &'lexer mut self,
    ) -> Result<&'lexer Vec<Token<'lexer>>, &'lexer Vec<LexError<'lexer>>> {
        // TODO: Implement states for lexer so we don't have to reset the stuff
        self.index = 0;
        self.tokens.clear();
        self.errs.clear();

        'lexer_loop: while let Some(ch) = self.get_cur_char() {
            if ch.is_whitespace() {
                self.index += 1;
                continue 'lexer_loop;
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
                        self.tokens.push(operator.clone());
                    } else {
                        unreachable!();
                    }
                }

                '<' => {
                    if let Some('=') = self.get_next_char() {
                        self.tokens.push(Token::LesserEq);
                    } else {
                        self.tokens.push(Token::Lesser);
                    }
                }

                '>' => {
                    if let Some('=') = self.get_next_char() {
                        self.tokens.push(Token::GreaterEq);
                    } else {
                        self.tokens.push(Token::Greater);
                    }
                }

                '=' => {
                    if let Some('=') = self.get_next_char() {
                        self.tokens.push(Token::IsEq);
                        self.index += 1; // For the second =
                    } else {
                        self.tokens.push(Token::AssignEq);
                    }
                }

                '(' => {
                    self.tokens.push(Token::ParenOpen);
                }
                ')' => {
                    self.tokens.push(Token::ParenClose);
                }

                '{' => {
                    self.tokens.push(Token::ParenCurlyOpen);
                }
                '}' => {
                    self.tokens.push(Token::ParenCurlyClose);
                }

                ';' => {
                    self.tokens.push(Token::Seminolon);
                }

                _ => {
                    self.errs.push(LexError::new(
                        self.input,
                        LexErrorType::UnknownCharacter(self.index),
                    ));
                }
            }

            self.index += 1;
        }

        if self.errs.is_empty() {
            Ok(&self.tokens)
        } else {
            Err(&self.errs)
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

        let identifier = &self.input[start..self.index];

        if let Some(keyword) = KEYWORDS.get(identifier) {
            self.tokens.push(keyword.clone());
        } else {
            self.tokens.push(Token::Identifier(identifier));
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

        let number_str = &self.input[start..self.index];

        if found_dot {
            match number_str.parse::<f64>() {
                Ok(number) => {
                    self.tokens.push(Token::FloatLiteral(number));
                }
                Err(_) => {
                    unreachable!();
                }
            }
        } else {
            match number_str.parse::<i128>() {
                Ok(number) => {
                    self.tokens.push(Token::IntLiteral(number));
                }
                Err(_) => {
                    unreachable!();
                }
            }
        }
    }
}
