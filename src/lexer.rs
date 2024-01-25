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

    fn read_number(&mut self) -> String {
        let initial_pointer = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[initial_pointer..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\r' || self.ch == b'\n' || self.ch == b'\t' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        self.skip_whitespace();
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
            ch if b'_' == ch || ch.is_ascii_alphabetic() => {
                let literal = self.read_identifier();
                tok = Token {
                    r#type: Token::lookup_ident(&literal),
                    literal,
                }
            }
            ch if ch.is_ascii_digit() => {
                tok = Token {
                    r#type: TokenType::INT,
                    literal: self.read_number(),
                }
            }
            _ => {
                tok = Token {
                    r#type: TokenType::ILLEGAL,
                    literal: self.ch.to_string(),
                }
            }
        }
        if tok.r#type != TokenType::INT
            && tok.r#type != TokenType::IDENT
            && tok.r#type != TokenType::LET
            && tok.r#type != TokenType::FUNCTION
        {
            self.read_char();
        }
        tok
    }

    pub fn read_identifier(&mut self) -> String {
        let initial_position = self.position;
        while b'_' == self.ch || self.ch.is_ascii_alphabetic() {
            self.read_char()
        }
        self.input[initial_position..self.position].to_string()
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
        let input = "let five = 5; let ten = 10; let add = fn(x, y) { x + y; }; let result = add(five, ten);";
        let tests: Vec<TestTokenType> = vec![
            TestTokenType {
                expected_token_type: TokenType::LET,
                expected_literal: "let".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::IDENT,
                expected_literal: "five".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::ASSIGN,
                expected_literal: "=".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::INT,
                expected_literal: "5".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::LET,
                expected_literal: "let".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::IDENT,
                expected_literal: "ten".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::ASSIGN,
                expected_literal: "=".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::INT,
                expected_literal: "10".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::LET,
                expected_literal: "let".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::IDENT,
                expected_literal: "add".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::ASSIGN,
                expected_literal: "=".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::FUNCTION,
                expected_literal: "fn".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::LPAREN,
                expected_literal: "(".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::IDENT,
                expected_literal: "x".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::COMMA,
                expected_literal: ",".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::IDENT,
                expected_literal: "y".to_string(),
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
                expected_token_type: TokenType::IDENT,
                expected_literal: "x".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::PLUS,
                expected_literal: "+".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::IDENT,
                expected_literal: "y".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::RBRACE,
                expected_literal: "}".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::LET,
                expected_literal: "let".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::IDENT,
                expected_literal: "result".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::ASSIGN,
                expected_literal: "=".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::IDENT,
                expected_literal: "add".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::LPAREN,
                expected_literal: "(".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::IDENT,
                expected_literal: "five".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::COMMA,
                expected_literal: ",".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::IDENT,
                expected_literal: "ten".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::RPAREN,
                expected_literal: ")".to_string(),
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
            println!(
                "left: {:?}, right: {:?}",
                &recieved_token.r#type, &test_token.expected_token_type
            );
            println!(
                "left: {:?}, right: {:?}",
                &recieved_token.literal, &test_token.expected_literal
            );
            assert_eq!(recieved_token.r#type, test_token.expected_token_type);
            assert_eq!(recieved_token.literal, test_token.expected_literal);
        }
    }
}
