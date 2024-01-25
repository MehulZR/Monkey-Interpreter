use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LET,
    FUNCTION,
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
}
#[derive(Debug, PartialEq)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("fn", TokenType::FUNCTION);
        keywords.insert("let", TokenType::LET);
        keywords
    };
}
impl Token {
    pub fn lookup_ident(input: &str) -> TokenType {
        let mut token_type: TokenType = TokenType::IDENT;
        if let Some(r#type) = KEYWORDS.get(input) {
            token_type = r#type.clone();
        }
        token_type
    }
}
