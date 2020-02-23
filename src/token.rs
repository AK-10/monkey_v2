#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

// impl Eq for Token {
//     fn eq(&self, other: &Self) -> bool {
//         self.token_type == 
//     }
// }

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
