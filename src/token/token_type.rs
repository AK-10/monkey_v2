#[derive(Debug)]
enum TokenType {
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
}
