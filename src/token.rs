use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Eq, Hash, Debug, PartialEq, Clone)]
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
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    EQ,
    NOTEQ,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("fn", TokenType::FUNCTION);
        keywords.insert("let", TokenType::LET);
        keywords.insert("true", TokenType::TRUE);
        keywords.insert("false", TokenType::FALSE);
        keywords.insert("if", TokenType::IF);
        keywords.insert("else", TokenType::ELSE);
        keywords.insert("return", TokenType::RETURN);
        keywords
    };
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
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
