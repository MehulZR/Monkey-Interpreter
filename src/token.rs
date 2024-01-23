#[derive(Debug, PartialEq)]
pub enum TokenType {
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
