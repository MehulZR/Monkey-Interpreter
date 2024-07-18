use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::ast::*;
use crate::lexer::*;
use crate::token::*;

#[derive(PartialEq, PartialOrd, Clone)]
enum PrecedenceType {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

lazy_static! {
    static ref PRECEDENCES: HashMap<TokenType, PrecedenceType> = {
        let mut keywords = HashMap::new();
        keywords.insert(TokenType::EQ, PrecedenceType::EQUALS);
        keywords.insert(TokenType::NOTEQ, PrecedenceType::EQUALS);
        keywords.insert(TokenType::LT, PrecedenceType::LESSGREATER);
        keywords.insert(TokenType::GT, PrecedenceType::LESSGREATER);
        keywords.insert(TokenType::PLUS, PrecedenceType::SUM);
        keywords.insert(TokenType::MINUS, PrecedenceType::SUM);
        keywords.insert(TokenType::SLASH, PrecedenceType::PRODUCT);
        keywords.insert(TokenType::ASTERISK, PrecedenceType::PRODUCT);
        keywords
    };
}

struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
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
                program.statements.push(stmt);
            }
            self.next_token();
        }
        Some(program)
    }

    pub fn parse_statement(&mut self) -> Option<StatementTypes> {
        match self.cur_token.r#type {
            TokenType::LET => {
                if let Some(stmt) = self.parse_let_statement() {
                    return Some(StatementTypes::LETSTATEMENT(stmt));
                }
                None
            }
            TokenType::RETURN => {
                if let Some(stmt) = self.parse_return_statement() {
                    return Some(StatementTypes::RETURNSTATEMENT(stmt));
                }
                None
            }
            _ => {
                if let Some(stmt) = self.parse_exp_statement() {
                    return Some(StatementTypes::EXPRESSIONSTATEMENT(stmt));
                }
                None
            }
        }
    }

    pub fn parse_exp_statement(&mut self) -> Option<ExpressionStatement> {
        let stmt = ExpressionStatement {
            token: self.cur_token.clone(),
            expression: self.parse_expression(PrecedenceType::LOWEST),
        };

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token()
        };

        Some(stmt)
    }

    pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let mut stmt = LetStatement {
            token: self.cur_token.clone(),
            name: Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            },
            value: EXPRESSION::IDENTIFIER(Identifier {
                token: Token {
                    r#type: TokenType::IDENT,
                    literal: "".to_string(),
                },
                value: "".to_string(),
            }),
        };

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        stmt.name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(stmt)
    }

    pub fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let stmt = ReturnStatement {
            token: self.cur_token.clone(),
            return_value: EXPRESSION::IDENTIFIER(Identifier {
                token: Token {
                    r#type: TokenType::IDENT,
                    literal: "".to_string(),
                },
                value: "".to_string(),
            }),
        };

        self.next_token();

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(stmt)
    }

    pub fn parse_expression(&mut self, precedence: PrecedenceType) -> EXPRESSION {
        let mut left = match self.cur_token.r#type {
            TokenType::IDENT => self.pares_identifier(),
            TokenType::INT => self.parse_integer(),
            TokenType::BANG | TokenType::MINUS => self.parse_prefix_expression(),
            other => panic!("no prefix parse fn for {:?} defined", other),
        };

        while !self.peek_token_is(TokenType::SEMICOLON) && precedence < self.peek_precedence() {
            left = match self.peek_token.r#type {
                TokenType::PLUS
                | TokenType::MINUS
                | TokenType::SLASH
                | TokenType::ASTERISK
                | TokenType::EQ
                | TokenType::NOTEQ
                | TokenType::LT
                | TokenType::GT => {
                    self.next_token();
                    self.parse_infix_expression(left)
                }
                _ => left,
            }
        }

        left
    }

    fn pares_identifier(&self) -> EXPRESSION {
        EXPRESSION::IDENTIFIER(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        })
    }
    fn parse_integer(&mut self) -> EXPRESSION {
        let literal_val = match self.cur_token.literal.parse() {
            Ok(val) => val,
            Err(_) => {
                self.errors.push(format!(
                    "Could not parse {} as integer",
                    self.cur_token.literal
                ));
                0
            }
        };

        EXPRESSION::INTEGER(IntegerLiteral {
            token: self.cur_token.clone(),
            value: literal_val,
        })
    }

    fn parse_prefix_expression(&mut self) -> EXPRESSION {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        self.next_token();
        let exp = PrefixExpression {
            token,
            operator,
            right: Box::new(self.parse_expression(PrecedenceType::PREFIX)),
        };

        EXPRESSION::PREFIX(exp)
    }

    fn parse_infix_expression(&mut self, left: EXPRESSION) -> EXPRESSION {
        let precedence = self.cur_precedence();
        let cur_token = self.cur_token.clone();

        self.next_token();

        EXPRESSION::INFIX(InfixExpression {
            token: cur_token.clone(),
            operator: cur_token.literal.clone(),
            left: Box::new(left),
            right: Box::new(self.parse_expression(precedence)),
        })
    }

    pub fn cur_token_is(&self, token: TokenType) -> bool {
        return self.cur_token.r#type == token;
    }

    pub fn peek_token_is(&self, token: TokenType) -> bool {
        return self.peek_token.r#type == token;
    }

    pub fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_is(token.clone()) {
            self.next_token();
            return true;
        } else {
            self.peek_errors(token);
            return false;
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    pub fn peek_errors(&mut self, expected_token_type: TokenType) {
        self.errors.push(format!(
            "Expected next token to be {:?}, got {:?}",
            expected_token_type, self.peek_token.r#type
        ))
    }

    pub fn peek_precedence(&self) -> PrecedenceType {
        match PRECEDENCES.get(&self.peek_token.r#type) {
            Some(p) => p.clone(),
            None => PrecedenceType::LOWEST,
        }
    }

    pub fn cur_precedence(&self) -> PrecedenceType {
        match PRECEDENCES.get(&self.cur_token.r#type) {
            Some(p) => p.clone(),
            None => PrecedenceType::LOWEST,
        }
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
            panic!(
                "program.statements doesn't contain 3 statements. got {}",
                program.statements.len()
            )
        }

        let tests: Vec<String> = vec!["x".to_string(), "y".to_string(), "foobar".to_string()];

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
            panic!(
                "program.statements doesn't contain 3 statements. got {}",
                program.statements.len()
            )
        }

        let tests: Vec<String> = vec!["x".to_string(), "y".to_string(), "foobar".to_string()];

        for i in 0..tests.len() {
            let statement = match program.statements.get(i) {
                Some(s) => s,
                None => panic!("Statement not found"),
            };
            if !test_return_statement(&statement) {
                panic!("Test failed! Ohh yeahh!")
            }
        }

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
    }

    #[test]
    fn test_identifier_expession() {
        let input = "foobar";

        let mut l = Lexer::new(input.to_string());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(&p);
        let program = match program {
            Some(p) => p,
            None => panic!("parse_program returned nil"),
        };

        if program.statements.len() != 1 {
            panic!(
                "program has not enough statments. got: {}",
                program.statements.len()
            )
        };

        let stmt = match &program.statements[0] {
            StatementTypes::EXPRESSIONSTATEMENT(s) => s,
            other => panic!(
                "program.statements[0] is not ExpressionStatement. got: {:?}",
                other
            ),
        };

        let exp = match &stmt.expression {
            EXPRESSION::IDENTIFIER(obj) => obj,
            _ => panic!("program.statement.expression is not of type identifier"),
        };

        if exp.token.r#type != TokenType::IDENT {
            panic!("Exp not Identifier. got: {:?}", exp.token.r#type)
        };

        if exp.value != "foobar".to_string() {
            panic!("ident.value not foobar. got: {}", exp.value)
        };

        if exp.token_literal() != "foobar".to_string() {
            panic!(
                "ident.TokenLiteral not foobar. got: {}",
                exp.token_literal()
            )
        };
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let mut l = Lexer::new(input.to_string());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(&p);
        let program = match program {
            Some(p) => p,
            None => panic!("parse_program returned nil"),
        };

        if program.statements.len() != 1 {
            panic!(
                "program has not enough statments. got: {}",
                program.statements.len()
            )
        };

        let stmt = match &program.statements[0] {
            StatementTypes::EXPRESSIONSTATEMENT(s) => s,
            other => panic!(
                "program.statements[0] is not ExpressionStatement. got: {:?}",
                other
            ),
        };

        let exp = match &stmt.expression {
            EXPRESSION::INTEGER(obj) => obj,
            _ => panic!("program.statement.expression is not of type integer"),
        };

        if exp.token.r#type != TokenType::INT {
            panic!("Exp not integer literal. got: {:?}", exp.token.r#type)
        };

        if exp.value != 5 {
            panic!("ident.value not 5. got: {}", exp.value)
        };

        if exp.token_literal() != "5".to_string() {
            panic!("ident.TokenLiteral not 5. got: {}", exp.token_literal())
        };
    }

    fn test_integer_literal(exp: &EXPRESSION, val: i64) {
        let integer_literal = match exp {
            EXPRESSION::INTEGER(obj) => obj,
            _ => panic!("exp not IntegerLiteral. Got {:?}", exp),
        };

        if integer_literal.value != val {
            panic!(
                "IntegerLiteral value not {}, Got {}",
                val, integer_literal.value
            )
        }

        if integer_literal.token_literal() != format!("{}", val) {
            panic!(
                "integerLiteral token_literal not {}. Got {}",
                val,
                integer_literal.token_literal()
            )
        }
    }

    #[test]
    fn test_prefix_operator_expression() {
        struct Test {
            input: String,
            operator: String,
            integer_val: i64,
        }
        let tests = [
            Test {
                input: "!5".into(),
                operator: "!".into(),
                integer_val: 5,
            },
            Test {
                input: "-15".into(),
                operator: "-".into(),
                integer_val: 15,
            },
        ];

        for test in tests.iter() {
            let input = test.input.clone();
            let mut l = Lexer::new(input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            check_parser_errors(&p);
            let program = match program {
                Some(p) => p,
                None => panic!("parse_program returned nil"),
            };

            if program.statements.len() != 1 {
                panic!(
                    "program has not enough statments. got: {}",
                    program.statements.len()
                )
            };
            let stmt = match &program.statements[0] {
                StatementTypes::EXPRESSIONSTATEMENT(s) => s,
                other => panic!(
                    "program.statements[0] is not ExpressionStatement. got: {:?}",
                    other
                ),
            };

            let exp = match &stmt.expression {
                EXPRESSION::PREFIX(obj) => obj,
                other => panic!(
                    "program.statement.expression is not of type prefix operator. Got {:?}",
                    other
                ),
            };

            if exp.operator != test.operator {
                panic!(
                    "Exp operator is not {:?}. got: {:?}",
                    test.operator, exp.operator
                )
            };

            test_integer_literal(exp.right.as_ref(), test.integer_val)
        }
    }

    #[test]
    fn test_infix_operator_expression() {
        struct Test {
            input: String,
            operator: String,
            left_val: i64,
            right_val: i64,
        }
        let tests = [
            Test {
                input: "5 + 5".into(),
                operator: "+".into(),
                left_val: 5,
                right_val: 5,
            },
            Test {
                input: "5 - 5".into(),
                operator: "-".into(),
                left_val: 5,
                right_val: 5,
            },
            Test {
                input: "5 * 5".into(),
                operator: "*".into(),
                left_val: 5,
                right_val: 5,
            },
            Test {
                input: "5 / 5".into(),
                operator: "/".into(),
                left_val: 5,
                right_val: 5,
            },
            Test {
                input: "5 > 5".into(),
                operator: ">".into(),
                left_val: 5,
                right_val: 5,
            },
            Test {
                input: "5 < 5".into(),
                operator: "<".into(),
                left_val: 5,
                right_val: 5,
            },
            Test {
                input: "5 == 5".into(),
                operator: "==".into(),
                left_val: 5,
                right_val: 5,
            },
            Test {
                input: "5 != 5".into(),
                operator: "!=".into(),
                left_val: 5,
                right_val: 5,
            },
        ];

        for test in tests.iter() {
            let input = test.input.clone();
            let mut l = Lexer::new(input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            check_parser_errors(&p);
            let program = match program {
                Some(p) => p,
                None => panic!("parse_program returned nil"),
            };

            if program.statements.len() != 1 {
                panic!(
                    "program has not enough statments. got: {}",
                    program.statements.len()
                )
            };
            let stmt = match &program.statements[0] {
                StatementTypes::EXPRESSIONSTATEMENT(s) => s,
                other => panic!(
                    "program.statements[0] is not ExpressionStatement. got: {:?}",
                    other
                ),
            };

            let exp = match &stmt.expression {
                EXPRESSION::INFIX(obj) => obj,
                other => panic!(
                    "program.statement.expression is not of type prefix operator. Got {:?}",
                    other
                ),
            };

            if exp.operator != test.operator {
                panic!(
                    "Exp operator is not {:?}. got: {:?}",
                    test.operator, exp.operator
                )
            };

            test_integer_literal(exp.left.as_ref(), test.left_val);
            test_integer_literal(exp.right.as_ref(), test.right_val);
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        struct Test {
            input: String,
            expected: String,
        }

        let tests = [
            Test {
                input: "-a * b".to_string(),
                expected: "((-a) * b)".to_string(),
            },
            Test {
                input: "!-a".to_string(),
                expected: "(!(-a))".to_string(),
            },
            Test {
                input: "a + b + c".to_string(),
                expected: "((a + b) + c)".to_string(),
            },
            Test {
                input: "a + b - c".to_string(),
                expected: "((a + b) - c)".to_string(),
            },
            Test {
                input: "a * b * c".to_string(),
                expected: "((a * b) * c)".to_string(),
            },
            Test {
                input: "a * b / c".to_string(),
                expected: "((a * b) / c)".to_string(),
            },
            Test {
                input: "a + b / c".to_string(),
                expected: "(a + (b / c))".to_string(),
            },
            Test {
                input: "a + b * c + d / e - f".to_string(),
                expected: "(((a + (b * c)) + (d / e)) - f)".to_string(),
            },
            Test {
                input: "3 + 4; -5 * 5".to_string(),
                expected: "(3 + 4)((-5) * 5)".to_string(),
            },
            Test {
                input: "5 > 4 == 3 < 4".to_string(),
                expected: "((5 > 4) == (3 < 4))".to_string(),
            },
            Test {
                input: "5 < 4 != 3 > 4".to_string(),
                expected: "((5 < 4) != (3 > 4))".to_string(),
            },
            Test {
                input: "3 + 4 * 5 == 3 * 1 + 4 * 5".to_string(),
                expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))".to_string(),
            },
        ];

        for test in tests {
            let input = test.input.clone();
            let mut l = Lexer::new(input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();

            check_parser_errors(&p);

            let program = match program {
                Some(p) => p,
                None => panic!("parse_program returned nil"),
            };

            let actual = program.string();
            if actual != test.expected {
                panic!("Expected {}, Got {}", test.expected, actual)
            }
        }
    }
}
