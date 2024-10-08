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

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let mut skip = false;
        let tok = match self.ch {
            b';' => Token {
                r#type: TokenType::SEMICOLON,
                literal: ';'.to_string(),
            },
            b'(' => Token {
                r#type: TokenType::LPAREN,
                literal: '('.to_string(),
            },
            b')' => Token {
                r#type: TokenType::RPAREN,
                literal: ')'.to_string(),
            },
            b',' => Token {
                r#type: TokenType::COMMA,
                literal: ','.to_string(),
            },
            b'+' => Token {
                r#type: TokenType::PLUS,
                literal: '+'.to_string(),
            },
            b'{' => Token {
                r#type: TokenType::LBRACE,
                literal: '{'.to_string(),
            },
            b'}' => Token {
                r#type: TokenType::RBRACE,
                literal: '}'.to_string(),
            },
            b'[' => Token {
                r#type: TokenType::LBRACKET,
                literal: '['.to_string(),
            },
            b']' => Token {
                r#type: TokenType::RBRACKET,
                literal: ']'.to_string(),
            },
            b'-' => Token {
                r#type: TokenType::MINUS,
                literal: '-'.to_string(),
            },
            b'/' => Token {
                r#type: TokenType::SLASH,
                literal: '/'.to_string(),
            },
            b'*' => Token {
                r#type: TokenType::ASTERISK,
                literal: '*'.to_string(),
            },
            b'<' => Token {
                r#type: TokenType::LT,
                literal: '<'.to_string(),
            },
            b'>' => Token {
                r#type: TokenType::GT,
                literal: '>'.to_string(),
            },
            b':' => Token {
                r#type: TokenType::COLON,
                literal: ":".to_string(),
            },
            0 => Token {
                r#type: TokenType::EOF,
                literal: "".to_string(),
            },
            b'"' => Token {
                r#type: TokenType::STRING,
                literal: self.read_string(),
            },
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token {
                        r#type: TokenType::NOTEQ,
                        literal: "!=".to_string(),
                    }
                } else {
                    Token {
                        r#type: TokenType::BANG,
                        literal: '!'.to_string(),
                    }
                }
            }
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token {
                        r#type: TokenType::EQ,
                        literal: "==".to_string(),
                    }
                } else {
                    Token {
                        r#type: TokenType::ASSIGN,
                        literal: '='.to_string(),
                    }
                }
            }
            ch if b'_' == ch || ch.is_ascii_alphabetic() => {
                skip = true;
                let literal = self.read_identifier();
                Token {
                    r#type: Token::lookup_ident(&literal),
                    literal,
                }
            }
            ch if ch.is_ascii_digit() => {
                skip = true;
                Token {
                    r#type: TokenType::INT,
                    literal: self.read_number(),
                }
            }
            _ => Token {
                r#type: TokenType::ILLEGAL,
                literal: self.ch.to_string(),
            },
        };
        if !skip {
            self.read_char();
        }
        tok
    }

    fn read_string(&mut self) -> String {
        let initial_positon = self.position + 1;
        loop {
            self.read_char();
            if self.ch == b'"' || self.ch == b'0' {
                break;
            }
        }

        self.input[initial_positon..self.position].to_string()
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
        let input = "let five = 5;
                     let ten = 10;
                     let add = fn(x, y) {
                        x + y;
                     };
                     let result = add(five, ten);
                     !-/*5;
                     5 < 10 > 5; 
                     if (5 < 10){
                        return true;
                     } else {
                        return false;
                     }
                     10 == 10;
                     10 != 9;
                     \"foobar\"
                     \"foo bar\"
                     [1, 2];
                     {\"foo\": \"bar\"}";

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
                expected_token_type: TokenType::BANG,
                expected_literal: "!".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::MINUS,
                expected_literal: "-".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::SLASH,
                expected_literal: "/".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::ASTERISK,
                expected_literal: "*".to_string(),
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
                expected_token_type: TokenType::INT,
                expected_literal: "5".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::LT,
                expected_literal: "<".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::INT,
                expected_literal: "10".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::GT,
                expected_literal: ">".to_string(),
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
                expected_token_type: TokenType::IF,
                expected_literal: "if".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::LPAREN,
                expected_literal: "(".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::INT,
                expected_literal: "5".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::LT,
                expected_literal: "<".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::INT,
                expected_literal: "10".to_string(),
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
                expected_token_type: TokenType::RETURN,
                expected_literal: "return".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::TRUE,
                expected_literal: "true".to_string(),
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
                expected_token_type: TokenType::ELSE,
                expected_literal: "else".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::LBRACE,
                expected_literal: "{".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::RETURN,
                expected_literal: "return".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::FALSE,
                expected_literal: "false".to_string(),
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
                expected_token_type: TokenType::INT,
                expected_literal: "10".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::EQ,
                expected_literal: "==".to_string(),
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
                expected_token_type: TokenType::INT,
                expected_literal: "10".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::NOTEQ,
                expected_literal: "!=".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::INT,
                expected_literal: "9".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::STRING,
                expected_literal: "foobar".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::STRING,
                expected_literal: "foo bar".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::LBRACKET,
                expected_literal: "[".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::INT,
                expected_literal: "1".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::COMMA,
                expected_literal: ",".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::INT,
                expected_literal: "2".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::RBRACKET,
                expected_literal: "]".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::LBRACE,
                expected_literal: "{".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::STRING,
                expected_literal: "foo".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::COLON,
                expected_literal: ":".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::STRING,
                expected_literal: "bar".to_string(),
            },
            TestTokenType {
                expected_token_type: TokenType::RBRACE,
                expected_literal: "}".to_string(),
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
