#[test]
fn next_token_test() {
    let input = "=+(){},;";
    let lexer = new(input);

    let expecteds = vec![
        Token {
            token_type: token::TokenType.Assign,
            literal: "=",
        },
        Token {
            token_type: token::TokenType.Plus,
            literal: "+",
        },
        Token {
            token_type: token::TokenType.Lparen,
            literal: "(",
        },
        Token {
            token_type: token::TokenType.Rparen,
            literal: ")",
        },
        Token {
            token_type: token::TokenType.Lbrace,
            literal: "{",
        },
        Token {
            token_type: token::TokenType.Rbrace,
            literal: "}",
        },
        Token {
            token_type: token::TokenType.Comma,
            literal: ",",
        },
        Token {
            token_type: token::TokenType.Semicolon,
            literal: ";",
        },
    ];

    for expected in expecteds {
        assert_eq!(lexer.next_token(), expected);
    }
}
