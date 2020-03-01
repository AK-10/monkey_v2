#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}


#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,

    // identifier + literal
    Ident,
    Int,

    // operator
    Assign,
    Plus,

    // delimiter
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // keyword
    Function,
    Let,
    // CallCC,
}
