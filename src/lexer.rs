use crate::tokens::token::Token;
use crate::tokens::token_type::TokenType;

#[derive(Debug)]
struct Lexer {
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

    fn read_char(&self) {
        self.current_position = self.read_position;
        if self.read_position < self.input_chars.len() {
            self.current_char = Some(self.input_chars[self.read_position]);
            self.read_position += 1;
        } else {
            self.current_char = None;
        }
    }

    fn next_token(&self) -> Token {
        let token = self.create_token();
        self.read_char();

        token
    }

    fn create_token(&self) -> Token {
        match self.current_char {
            Some(char) => {
                match char {
                    '=' => Token {
                        token_type: tokens::token_type::TokenType::Assign,
                        literal: char.to_string(),
                    },
                    ';' => Token {
                        token_type: token::TokenType.Semicolon,
                        literal: char.to_string(),
                    },
                    '(' => Token {
                        token_type: token::TokenType.Lparen,
                        literal: char.to_string(),
                    },
                    ')' => Token {
                        token_type: token::TokenType.Rparen,
                        literal: char.to_string(),
                    },
                    ',' => Token {
                        token_type: token::TokenType.Comma,
                        literal: char.to_string(),
                    },
                    '+' => Token {
                        token_type: token::TokenType.Plus,
                        literal: char.to_string(),
                    },
                    '{' => Token {
                        token_type: token::TokenType.Lbrace,
                        literal: char.to_string(),
                    },
                    '}' => Token {
                        token_type: token::TokenType.Rbrace,
                        literal: char.to_string(),
                    },
                    _ => Token {
                        token_type: token::TokenType.Illegal,
                        literal: char.to_string(),
                    },
                }
            },
            None => Token {
                token_type: token::TokenType.Eof,
                literal: "".to_string(),
            },
        }
    }
}