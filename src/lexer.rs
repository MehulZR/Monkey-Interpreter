use crate::token::*;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}
impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input.as_bytes()[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1
    }
    pub fn next_token(&mut self) -> Token {
        let tok: Token;

        match self.ch {
            b'=' => {
                tok = Token {
                    r#type: TokenType::ASSIGN,
                    literal: '='.to_string(),
                }
            }
            b';' => {
                tok = Token {
                    r#type: TokenType::SEMICOLON,
                    literal: ';'.to_string(),
                }
            }
            b'(' => {
                tok = Token {
                    r#type: TokenType::LPAREN,
                    literal: '('.to_string(),
                }
            }
            b')' => {
                tok = Token {
                    r#type: TokenType::RPAREN,
                    literal: ')'.to_string(),
                }
            }
            b',' => {
                tok = Token {
                    r#type: TokenType::COMMA,
                    literal: ','.to_string(),
                }
            }
            b'+' => {
                tok = Token {
                    r#type: TokenType::PLUS,
                    literal: '+'.to_string(),
                }
            }
            b'{' => {
                tok = Token {
                    r#type: TokenType::LBRACE,
                    literal: '{'.to_string(),
                }
            }
            b'}' => {
                tok = Token {
                    r#type: TokenType::RBRACE,
                    literal: '}'.to_string(),
                }
            }
            0 => {
                tok = Token {
                    r#type: TokenType::EOF,
                    literal: "".to_string(),
                }
            }
            _ => {
                tok = Token {
                    r#type: TokenType::EOF,
                    literal: "".to_string(),
                }
            }
        }
        self.read_char();
        tok
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    struct TestTokenType {
        expected_token_type: TokenType,
        expected_literal: String,
    }
    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests: Vec<TestTokenType> = vec![
            TestTokenType {
                expected_token_type: TokenType::ASSIGN,
                expected_literal: "=".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::PLUS,
                expected_literal: "+".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::LPAREN,
                expected_literal: "(".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::RPAREN,
                expected_literal: ")".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::LBRACE,
                expected_literal: "{".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::RBRACE,
                expected_literal: "}".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::COMMA,
                expected_literal: ",".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::EOF,
                expected_literal: "".to_string(),
            },
        ];
        let mut test_iter = Lexer::new(input.to_string());
        for test_token in tests.iter() {
            let recieved_token = test_iter.next_token();
            assert_eq!(recieved_token.r#type, test_token.expected_token_type);
            assert_eq!(recieved_token.literal, test_token.expected_literal);
        }
    }
}
