use super::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: super::token_type::TokenType,
    literal: String,
}
