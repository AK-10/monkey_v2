use crate::token::{ Token, TokenType };

#[derive(Debug)]
pub struct Lexer {
    input: String,
    current_position: usize,
    read_position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input,
            current_position: 0,
            read_position: 0,
            current_char: None,
        };
        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        self.current_position = self.read_position;
        if self.read_position < self.input.len() {
            self.current_char = self.input.chars().nth(self.read_position);
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
            Some(cur) => {
                match cur {
                    '=' => Token {
                        token_type: TokenType::Assign,
                        literal: cur.to_string(),
                    },
                    ';' => Token {
                        token_type: TokenType::Semicolon,
                        literal: cur.to_string(),
                    },
                    '(' => Token {
                        token_type: TokenType::LParen,
                        literal: cur.to_string(),
                    },
                    ')' => Token {
                        token_type: TokenType::RParen,
                        literal: cur.to_string(),
                    },
                    ',' => Token {
                        token_type: TokenType::Comma,
                        literal: cur.to_string(),
                    },
                    '+' => Token {
                        token_type: TokenType::Plus,
                        literal: cur.to_string(),
                    },
                    '{' => Token {
                        token_type: TokenType::LBrace,
                        literal: cur.to_string(),
                    },
                    '}' => Token {
                        token_type: TokenType::RBrace,
                        literal: cur.to_string(),
                    },
                    _ => {
                        if is_letter(cur) {
                            let ident_literal = self.read_identifier();
                            return Token {
                                token_type: TokenType::Ident,
                                literal: ident_literal,
                            }
                        } else {
                            Token {
                                token_type: TokenType::Illegal,
                                literal: "".to_string(),
                            }
                        }
                    },
                }
            },
            None => Token {
                token_type: TokenType::Eof,
                literal: "".to_string(),
            },
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.current_position;
        while self.current_char.map(is_letter).unwrap_or(false) {
            self.read_char()
        };

        let literal_chars = &self.input[position..self.current_position];
        
        String::from(literal_chars)
    }
}

fn is_letter(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}

#[test]
fn next_token_test() {
    let input = "\
let five = 5;
let ten = 10

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten)";

    let mut lexer = Lexer::new(input.to_string());

    let expecteds = vec![
        Token {
            token_type: TokenType::Let,
            literal: "let".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "five".to_string(),
        },
        Token {
            token_type: TokenType::Assign,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::Int,
            literal: "5".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::Let,
            literal: "let".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "ten".to_string(),
        },
        Token {
            token_type: TokenType::Assign,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::Int,
            literal: "10".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::Let,
            literal: "let".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "add".to_string(),
        },
        Token {
            token_type: TokenType::Assign,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::Function,
            literal: "fn".to_string(),
        },
        Token {
            token_type: TokenType::LParen,
            literal: "(".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "x".to_string(),
        },
        Token {
            token_type: TokenType::Comma,
            literal: ",".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "y".to_string(),
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
            token_type: TokenType::Ident,
            literal: "x".to_string(),
        },
        Token {
            token_type: TokenType::Plus,
            literal: "+".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "y".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::RBrace,
            literal: "}".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::Let,
            literal: "let".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "result".to_string(),
        },
        Token {
            token_type: TokenType::Assign,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "add".to_string(),
        },
        Token {
            token_type: TokenType::LParen,
            literal: "(".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "five".to_string(),
        },
        Token {
            token_type: TokenType::Comma,
            literal: ",".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "ten".to_string(),
        },
        Token {
            token_type: TokenType::RParen,
            literal: ")".to_string(),
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
