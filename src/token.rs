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
