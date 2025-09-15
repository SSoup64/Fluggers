#[cfg(test)]
mod lexer {
    use crate::lexer::Lexer;
    use crate::token::Token;

    fn test_lexer(code: String, expected: Vec<Token>) {
        let lexer = Lexer::new(&code);
        let tokens = lexer.into_tokens().map_err(|errs| {
            for err in errs {
                eprintln!("{}", err);
            }
            "Failed at lexing"
        }).unwrap();

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_operators() {
        test_lexer(
            String::from("+ - / * = == < > <= >="),
            vec![
                Token::Plus,
                Token::Minus,
                Token::Slash,
                Token::Star,
                Token::AssignEq,
                Token::IsEq,
                Token::Lesser,
                Token::Greater,
                Token::LesserEq,
                Token::GreaterEq,
            ]
        );
    }

    #[test]
    fn test_operators_no_spaces() {
        test_lexer(
            String::from("+-/*= ==<><=>="),
            vec![
                Token::Plus,
                Token::Minus,
                Token::Slash,
                Token::Star,
                Token::AssignEq,
                Token::IsEq,
                Token::Lesser,
                Token::Greater,
                Token::LesserEq,
                Token::GreaterEq,
            ]
        );
    }

    #[test]
    fn test_literals() {
        test_lexer(
            String::from("hello 4 5test foo4 foo_Bar_Buzz 4.2"),
            vec![
                Token::Identifier("hello"),
                Token::IntLiteral(4),
                Token::IntLiteral(5),
                Token::Identifier("test"),
                Token::Identifier("foo4"),
                Token::Identifier("foo_Bar_Buzz"),
                Token::FloatLiteral(4.2),
            ]
        )
    }
}










