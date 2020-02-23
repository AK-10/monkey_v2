use crate::token::{ Token, TokenType };

#[derive(Debug)]
pub struct Lexer {
    input_chars: Vec<char>,
    current_position: usize,
    read_position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input_chars: input.chars().collect(),
            current_position: 0,
            read_position: 0,
            current_char: None,
        };
        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        self.current_position = self.read_position;
        if self.read_position < self.input_chars.len() {
            self.current_char = Some(self.input_chars[self.read_position]);
            self.read_position += 1;
        } else {
            self.current_char = None;
        }
    }

    fn next_token(&mut self) -> Token {
        let token = self.create_token();
        self.read_char();

        token
    }

    fn create_token(&mut self) -> Token {
        match self.current_char {
            Some(char) => {
                match char {
                    '=' => Token {
                        token_type: TokenType::Assign,
                        literal: char.to_string(),
                    },
                    ';' => Token {
                        token_type: TokenType::Semicolon,
                        literal: char.to_string(),
                    },
                    '(' => Token {
                        token_type: TokenType::LParen,
                        literal: char.to_string(),
                    },
                    ')' => Token {
                        token_type: TokenType::RParen,
                        literal: char.to_string(),
                    },
                    ',' => Token {
                        token_type: TokenType::Comma,
                        literal: char.to_string(),
                    },
                    '+' => Token {
                        token_type: TokenType::Plus,
                        literal: char.to_string(),
                    },
                    '{' => Token {
                        token_type: TokenType::LBrace,
                        literal: char.to_string(),
                    },
                    '}' => Token {
                        token_type: TokenType::RBrace,
                        literal: char.to_string(),
                    },
                    _ => Token {
                        token_type: TokenType::Illegal,
                        literal: char.to_string(),
                    },
                }
            },
            None => Token {
                token_type: TokenType::Eof,
                literal: "".to_string(),
            },
        }
    }

}

#[test]
fn next_token_test() {
    let input = "=+(){},;";
    let mut lexer = Lexer::new(input.to_string());

    let expecteds = vec![
        Token {
            token_type: TokenType::Assign,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::Plus,
            literal: "+".to_string(),
        },
        Token {
            token_type: TokenType::LParen,
            literal: "(".to_string(),
        },
        Token {
            token_type: TokenType::RParen,
            literal: ")".to_string(),
        },
        Token {
            token_type: TokenType::LBrace,
            literal: "{".to_string(),
        },
        Token {
            token_type: TokenType::RBrace,
            literal: "}".to_string(),
        },
        Token {
            token_type: TokenType::Comma,
            literal: ",".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
    ];

    for expected in expecteds {
        assert_eq!(lexer.next_token(), expected);
    }
}

