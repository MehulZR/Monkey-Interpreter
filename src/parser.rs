use std::collections::HashMap;

use crate::ast::Expression;
use crate::ast::Identifier;
use crate::ast::LetStatement;
use crate::ast::Program;
use crate::ast::ReturnStatement;
use crate::ast::StatementTypes;
use crate::lexer::*;
use crate::token::*;

struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, fn() -> Expression>,
    infix_parse_fns: HashMap<TokenType, fn(exp: Expression) -> Expression>,
}

impl Parser<'_> {
    pub fn new(l: &mut Lexer) -> Parser {
        let mut p = Parser {
            l,
            cur_token: Token {
                r#type: TokenType::EOF,
                literal: "".to_string(),
            },
            peek_token: Token {
                r#type: TokenType::EOF,
                literal: "".to_string(),
            },
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program { statements: vec![] };
        while self.cur_token.r#type != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                println!("statement: {:#?}", stmt);
                program.statements.push(stmt);
            }
            self.next_token();
        }
        Some(program)
    }

    pub fn parse_statement(&mut self) -> Option<StatementTypes> {
        match self.cur_token.r#type {
            TokenType::LET => Some(self.parse_letstatement()),
            TokenType::RETURN => Some(self.parse_return_statement()),
            _ => None,
        }
    }

    pub fn parse_letstatement(&mut self) -> StatementTypes {
        let mut stmt = LetStatement {
            token: self.cur_token.clone(),
            name: Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            },
            value: Expression {},
        };

        if !self.expect_peek(TokenType::IDENT) {
            // return None;
        }

        stmt.name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            //     return None;
        }

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        // Some(StatementTypes::LETSTATEMENT(stmt))
        StatementTypes::LETSTATEMENT(stmt)
    }

    pub fn parse_return_statement(&mut self) -> StatementTypes {
        let stmt = ReturnStatement {
            token: self.cur_token.clone(),
            return_value: Expression {},
        };

        self.next_token();

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        StatementTypes::RETURNSTATEMENT(stmt)
    }

    pub fn cur_token_is(&self, token: TokenType) -> bool {
        return self.cur_token.r#type == token;
    }

    pub fn peek_token_is(&self, token: TokenType) -> bool {
        return self.peek_token.r#type == token;
    }

    pub fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    pub fn peek_errors(&mut self, expected_token_type: Token) {
        self.errors.push(format!(
            "Expected next token to be {}, got {}",
            expected_token_type.literal,
            self.peek_token.literal.to_string(),
        ))
    }

    pub fn register_prefix(&mut self, token: TokenType, f: fn() -> Expression) {
        self.prefix_parse_fns.insert(token, f);
    }

    pub fn register_infix(&mut self, token: TokenType, f: fn(exp: Expression) -> Expression) {
        self.infix_parse_fns.insert(token, f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Node;
    use crate::ast::StatementTypes;
    use core::panic;

    fn check_parser_errors(p: &Parser) {
        let errors = p.errors();
        if errors.len() == 0 {
            return;
        }
        println!("Parser has {} errors", errors.len());
        for err in errors.iter() {
            println!("Parser error:{}", err);
        }
        panic!();
    }

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;
                 let y = 10;
                 let foobar = 838383;";
        let mut l = Lexer::new(input.to_string());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(&p);
        let program = match program {
            Some(p) => p,
            None => panic!("parse_program returned nil"),
        };
        if program.statements.len() != 3 {
            println!(
                "program.statements doesn't contain 3 statements. got {}",
                program.statements.len()
            )
        }
        let tests: Vec<String> = vec!["x".to_string(), "y".to_string(), "foobar".to_string()];
        fn test_let_statement(statement: &StatementTypes, expected_identifier: &String) -> bool {
            match statement {
                StatementTypes::LETSTATEMENT(stmt) => {
                    if stmt.name.value != expected_identifier.clone() {
                        return false;
                    }
                    if stmt.name.token_literal() != expected_identifier.clone() {
                        return false;
                    }
                }
                _ => panic!("Statement token literal not let"),
            }
            true
        }
        for i in 0..tests.len() {
            let current_test = tests.get(i).unwrap();
            let statement = match program.statements.get(i) {
                Some(s) => s,
                None => panic!("Statement not found"),
            };
            if !test_let_statement(&statement, current_test) {
                panic!("Test failed! Ohh yeahh!")
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let input = "return 5;
                    return 10;
                    return 993322;";
        let mut l = Lexer::new(input.to_string());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(&p);
        let program = match program {
            Some(p) => p,
            None => panic!("parse_program returned nil"),
        };
        if program.statements.len() != 3 {
            println!(
                "program.statements doesn't contain 3 statements. got {}",
                program.statements.len()
            )
        }
        let tests: Vec<String> = vec!["x".to_string(), "y".to_string(), "foobar".to_string()];
        fn test_return_statement(statement: &StatementTypes) -> bool {
            match statement {
                StatementTypes::RETURNSTATEMENT(stmt) => {
                    if stmt.token_literal() != "return" {
                        println!("huaaaaaaaaaaaa");
                        return false;
                    }
                }
                _ => panic!("Statement token literal not let"),
            }
            true
        }
        for i in 0..tests.len() {
            // let current_test = tests.get(i).unwrap();
            let statement = match program.statements.get(i) {
                Some(s) => s,
                None => panic!("Statement not found"),
            };
            if !test_return_statement(&statement) {
                panic!("Test failed! Ohh yeahh!")
            }
        }
    }
}
