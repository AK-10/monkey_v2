pub mod token {
    #[derive(Debug, Eq)]
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
    }

    #[derive(Debug)]
    pub struct Token {
        type: TokenType,
        literal: String,
    }
}
