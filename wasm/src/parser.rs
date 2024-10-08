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
    INDEX,
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
        keywords.insert(TokenType::LPAREN, PrecedenceType::CALL);
        keywords.insert(TokenType::LBRACKET, PrecedenceType::INDEX);
        keywords
    };
}

pub struct Parser<'a> {
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

    fn next_token(&mut self) {
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

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.r#type {
            TokenType::LET => {
                if let Some(stmt) = self.parse_let_statement() {
                    return Some(Statement::LETSTATEMENT(stmt));
                }
                None
            }
            TokenType::RETURN => {
                if let Some(stmt) = self.parse_return_statement() {
                    return Some(Statement::RETURNSTATEMENT(stmt));
                }
                None
            }
            _ => {
                if let Some(stmt) = self.parse_exp_statement() {
                    return Some(Statement::EXPRESSIONSTATEMENT(stmt));
                }
                None
            }
        }
    }

    fn parse_exp_statement(&mut self) -> Option<ExpressionStatement> {
        let stmt = ExpressionStatement {
            token: self.cur_token.clone(),
            expression: self.parse_expression(PrecedenceType::LOWEST),
        };

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token()
        };

        Some(stmt)
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let identifier = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        self.next_token();

        let stmt = LetStatement {
            token,
            name: identifier,
            value: self.parse_expression(PrecedenceType::LOWEST),
        };

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(stmt)
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let token = self.cur_token.clone();

        self.next_token();

        let stmt = ReturnStatement {
            token,
            return_value: self.parse_expression(PrecedenceType::LOWEST),
        };

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(stmt)
    }

    fn parse_grouped_expression(&mut self) -> EXPRESSION {
        self.next_token();

        let exp = self.parse_expression(PrecedenceType::LOWEST);

        self.next_token();

        exp
    }

    fn parse_expression(&mut self, precedence: PrecedenceType) -> EXPRESSION {
        let mut left = match self.cur_token.r#type {
            TokenType::IDENT => self.pares_identifier(),
            TokenType::INT => self.parse_integer(),
            TokenType::BANG | TokenType::MINUS => self.parse_prefix_expression(),
            TokenType::TRUE | TokenType::FALSE => self.parse_boolean(),
            TokenType::LPAREN => self.parse_grouped_expression(),
            TokenType::IF => self.parse_if_expression(),
            TokenType::FUNCTION => self.parse_fn_literal(),
            TokenType::STRING => self.parse_string_literal(),
            TokenType::LBRACKET => self.parse_array_literal(),
            TokenType::LBRACE => self.parse_hash_literal(),
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
                TokenType::LPAREN => {
                    self.next_token();
                    self.parse_call_expression(left)
                }
                TokenType::LBRACKET => {
                    self.next_token();
                    self.parse_index_expression(left)
                }
                _ => left,
            }
        }

        left
    }

    fn parse_hash_literal(&mut self) -> EXPRESSION {
        let cur_token = self.cur_token.clone();
        let mut pairs = vec![];

        while !self.peek_token_is(TokenType::RBRACE) {
            self.next_token();

            let key = self.parse_expression(PrecedenceType::LOWEST);

            if !self.expect_peek(TokenType::COLON) {
                panic!("Expected colon here. Got: {:?}", self.cur_token);
            };

            self.next_token();

            let val = self.parse_expression(PrecedenceType::LOWEST);

            pairs.push((key, val));

            if !self.peek_token_is(TokenType::RBRACE) && !self.expect_peek(TokenType::COMMA) {
                panic!("Expected COMMA or RBRACE here while parsing hash literal");
            }
        }

        if !self.expect_peek(TokenType::RBRACE) {
            panic!("Expected RBRACE here while parsing hash literal");
        }

        EXPRESSION::HashLiteral(HashLiteral {
            token: cur_token,
            pairs,
        })
    }

    fn parse_index_expression(&mut self, left: EXPRESSION) -> EXPRESSION {
        let cur_token = self.cur_token.clone();

        self.next_token();

        let index = self.parse_expression(PrecedenceType::LOWEST);

        if !self.expect_peek(TokenType::RBRACKET) {
            panic!("Expected RBracket while parsing array index expression");
        }

        EXPRESSION::IndexExpression(IndexExpression {
            token: cur_token,
            left: Box::new(left),
            index: Box::new(index),
        })
    }

    fn parse_if_expression(&mut self) -> EXPRESSION {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::LPAREN) {
            panic!("Left paran not found")
        }

        self.next_token();

        let condition = Box::new(self.parse_expression(PrecedenceType::LOWEST));

        if !self.expect_peek(TokenType::RPAREN) {
            panic!("Right paran not found")
        }

        if !self.expect_peek(TokenType::LBRACE) {
            panic!("Left brace not found")
        }

        let consequence = self.parse_block_statement();
        let mut alternative: Option<BlockStatement> = None;
        if self.peek_token_is(TokenType::ELSE) {
            self.next_token();

            if !self.expect_peek(TokenType::LBRACE) {
                panic!("left brace not found while parsing else part");
            }

            alternative = Some(self.parse_block_statement())
        }
        EXPRESSION::IF(IfExpression {
            token,
            condition,
            consequence,
            alternative,
        })
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let mut block_stmt = BlockStatement {
            token: self.cur_token.clone(),
            statements: vec![],
        };

        self.next_token();

        while !self.cur_token_is(TokenType::RBRACE) && !self.cur_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                block_stmt.statements.push(stmt);
                self.next_token();
            }
        }

        block_stmt
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

    fn parse_boolean(&self) -> EXPRESSION {
        EXPRESSION::BOOLEAN(BooleanExpression {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenType::TRUE),
        })
    }

    fn parse_string_literal(&self) -> EXPRESSION {
        EXPRESSION::StringLiteral(StringLiteral {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        })
    }

    fn parse_array_literal(&mut self) -> EXPRESSION {
        return EXPRESSION::ArrayLiteral(ArrayLitearl {
            token: self.cur_token.clone(),
            items: self.parse_expression_list(TokenType::RBRACKET),
        });
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Vec<EXPRESSION> {
        let mut list: Vec<EXPRESSION> = vec![];

        if self.peek_token_is(end) {
            self.next_token();
            return list;
        }

        self.next_token();

        list.push(self.parse_expression(PrecedenceType::LOWEST));

        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();

            list.push(self.parse_expression(PrecedenceType::LOWEST));
        }

        if !self.expect_peek(end) {
            panic!("Expected {:?} while parsing parameter list", end)
        }

        list
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

    fn parse_fn_literal(&mut self) -> EXPRESSION {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::LPAREN) {
            panic!("Expected LPAREN not found while parsing fn literal");
        }

        let parameters = self.parse_fn_params();

        if !self.expect_peek(TokenType::LBRACE) {
            panic!("Expected LBRACE not found while parsing fn literal");
        };

        EXPRESSION::FN(FnExpression {
            token,
            parameters,
            body: self.parse_block_statement(),
        })
    }

    fn parse_fn_params(&mut self) -> Vec<Identifier> {
        let mut params: Vec<Identifier> = vec![];

        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token();
            return params;
        }

        self.next_token();

        params.push(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();

            params.push(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            });
        }

        if !self.expect_peek(TokenType::RPAREN) {
            panic!("Expected RPAREN not found while parsing fn literal");
        }

        params
    }

    fn parse_call_expression(&mut self, function: EXPRESSION) -> EXPRESSION {
        EXPRESSION::CALL(CallExpression {
            token: self.cur_token.clone(),
            function: Box::new(function),
            args: self.parse_expression_list(TokenType::RPAREN),
        })
    }

    fn cur_token_is(&self, token: TokenType) -> bool {
        return self.cur_token.r#type == token;
    }

    fn peek_token_is(&self, token: TokenType) -> bool {
        return self.peek_token.r#type == token;
    }

    fn expect_peek(&mut self, token: TokenType) -> bool {
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

    fn peek_errors(&mut self, expected_token_type: TokenType) {
        self.errors.push(format!(
            "Expected next token to be {:?}, got {:?}",
            expected_token_type, self.peek_token.r#type
        ))
    }

    fn peek_precedence(&self) -> PrecedenceType {
        match PRECEDENCES.get(&self.peek_token.r#type) {
            Some(p) => p.clone(),
            None => PrecedenceType::LOWEST,
        }
    }

    fn cur_precedence(&self) -> PrecedenceType {
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
    use crate::ast::Statement;
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
        struct Test {
            input: String,
            expected_identifier: String,
            expected_val: String,
        }

        let tests = [
            Test {
                input: "let x = 5;".to_string(),
                expected_identifier: "x".to_string(),
                expected_val: "5".to_string(),
            },
            Test {
                input: "let y = true;".to_string(),
                expected_identifier: "y".to_string(),
                expected_val: "true".to_string(),
            },
            Test {
                input: "let foobar = y;".to_string(),
                expected_identifier: "foobar".to_string(),
                expected_val: "y".to_string(),
            },
        ];

        for test in tests {
            let mut l = Lexer::new(test.input.clone());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            check_parser_errors(&p);

            let program = match program {
                Some(p) => p,
                None => panic!("parse_program returned nil"),
            };

            if program.statements.len() != 1 {
                panic!(
                    "program.statements doesn't contain 1 statements. got {}",
                    program.statements.len()
                )
            }

            test_let_statement(&program.statements[0], &test.expected_identifier);

            let stmt = match &program.statements[0] {
                Statement::LETSTATEMENT(s) => s,
                other => panic!("expected stmt to be of type letExpression. Got {:?}", other),
            };

            test_literal_expression(&stmt.value, test.expected_val);
        }
    }

    fn test_let_statement(statement: &Statement, expected_identifier: &String) -> bool {
        match statement {
            Statement::LETSTATEMENT(stmt) => {
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

        fn test_return_statement(statement: &Statement) -> bool {
            match statement {
                Statement::RETURNSTATEMENT(stmt) => {
                    if stmt.token_literal() != "return" {
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
            Statement::EXPRESSIONSTATEMENT(s) => s,
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
            Statement::EXPRESSIONSTATEMENT(s) => s,
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

    #[test]
    fn test_prefix_operator_expression() {
        struct Test {
            input: String,
            operator: String,
            val: String,
        }
        let tests = [
            Test {
                input: "!5".into(),
                operator: "!".into(),
                val: 5.to_string(),
            },
            Test {
                input: "-15".into(),
                operator: "-".into(),
                val: 15.to_string(),
            },
            Test {
                input: "-15".into(),
                operator: "-".into(),
                val: 15.to_string(),
            },
            Test {
                input: "-15".into(),
                operator: "-".into(),
                val: 15.to_string(),
            },
            Test {
                input: "!true".into(),
                operator: "!".into(),
                val: true.to_string(),
            },
            Test {
                input: "!false".into(),
                operator: "!".into(),
                val: false.to_string(),
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
                Statement::EXPRESSIONSTATEMENT(s) => s,
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

            match exp.right.as_ref() {
                EXPRESSION::INTEGER(_) => {
                    test_integer_literal(&exp.right.as_ref(), test.val.parse().unwrap())
                }
                EXPRESSION::BOOLEAN(_) => {
                    test_boolean_literal(&exp.right.as_ref(), test.val.clone())
                }
                _ => panic!(""),
            }
        }
    }

    #[test]
    fn test_infix_operator_expression() {
        struct Test {
            input: String,
            operator: String,
            left_val: String,
            right_val: String,
        }
        let tests = [
            Test {
                input: "5 + 5".into(),
                operator: "+".into(),
                left_val: 5.to_string(),
                right_val: 5.to_string(),
            },
            Test {
                input: "5 - 5".into(),
                operator: "-".into(),
                left_val: 5.to_string(),
                right_val: 5.to_string(),
            },
            Test {
                input: "5 * 5".into(),
                operator: "*".into(),
                left_val: 5.to_string(),
                right_val: 5.to_string(),
            },
            Test {
                input: "5 / 5".into(),
                operator: "/".into(),
                left_val: 5.to_string(),
                right_val: 5.to_string(),
            },
            Test {
                input: "5 > 5".into(),
                operator: ">".into(),
                left_val: 5.to_string(),
                right_val: 5.to_string(),
            },
            Test {
                input: "5 < 5".into(),
                operator: "<".into(),
                left_val: 5.to_string(),
                right_val: 5.to_string(),
            },
            Test {
                input: "5 == 5".into(),
                operator: "==".into(),
                left_val: 5.to_string(),
                right_val: 5.to_string(),
            },
            Test {
                input: "5 != 5".into(),
                operator: "!=".into(),
                left_val: 5.to_string(),
                right_val: 5.to_string(),
            },
            Test {
                input: "true == true".into(),
                operator: "==".into(),
                left_val: true.to_string(),
                right_val: true.to_string(),
            },
            Test {
                input: "true != false".into(),
                operator: "!=".into(),
                left_val: true.to_string(),
                right_val: false.to_string(),
            },
            Test {
                input: "false == false".into(),
                operator: "==".into(),
                left_val: false.to_string(),
                right_val: false.to_string(),
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
                Statement::EXPRESSIONSTATEMENT(s) => s,
                other => panic!(
                    "program.statements[0] is not ExpressionStatement. got: {:?}",
                    other
                ),
            };

            test_infix_expression(
                &stmt.expression,
                test.left_val.to_string(),
                test.operator.clone(),
                test.right_val.to_string(),
            )
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
            Test {
                input: "true".to_string(),
                expected: "true".to_string(),
            },
            Test {
                input: "false".to_string(),
                expected: "false".to_string(),
            },
            Test {
                input: "3 > 5 == false".to_string(),
                expected: "((3 > 5) == false)".to_string(),
            },
            Test {
                input: "3 < 5 == true".to_string(),
                expected: "((3 < 5) == true)".to_string(),
            },
            Test {
                input: "1 + (2 + 3) + 4".to_string(),
                expected: "((1 + (2 + 3)) + 4)".to_string(),
            },
            Test {
                input: "(5 + 5) * 2".to_string(),
                expected: "((5 + 5) * 2)".to_string(),
            },
            Test {
                input: "2 / (5 + 5)".to_string(),
                expected: "(2 / (5 + 5))".to_string(),
            },
            Test {
                input: "-(5 + 5)".to_string(),
                expected: "(-(5 + 5))".to_string(),
            },
            Test {
                input: "!(true == true)".to_string(),
                expected: "(!(true == true))".to_string(),
            },
            Test {
                input: "a + add(b * c) + d".to_string(),
                expected: "((a + add((b * c))) + d)".to_string(),
            },
            Test {
                input: "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))".to_string(),
                expected: "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))".to_string(),
            },
            Test {
                input: "add(a + b + c * d / f + g)".to_string(),
                expected: "add((((a + b) + ((c * d) / f)) + g))".to_string(),
            },
            Test {
                input: "a * [1, 2, 3, 4][b * c] * d".to_string(),
                expected: "((a * ([1, 2, 3, 4][(b * c)])) * d)".to_string(),
            },
            Test {
                input: "add(a * b[2], b[1], 2 * [1, 2][1])".to_string(),
                expected: "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))".to_string(),
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

    #[test]
    fn test_boolean_expression() {
        let input = "true;";

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
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!(
                "program.statements[0] is not ExpressionStatement. got: {:?}",
                other
            ),
        };

        let exp = match &stmt.expression {
            EXPRESSION::BOOLEAN(obj) => obj,
            _ => panic!("program.statement.expression is not of type boolean"),
        };

        if exp.token.r#type != TokenType::TRUE && exp.token.r#type != TokenType::FALSE {
            panic!("Exp not boolean. got: {:?}", exp.token.r#type)
        };

        if exp.value != true {
            panic!("ident.value not true. got: {}", exp.value)
        };

        if exp.token_literal() != "true".to_string() {
            panic!("ident.TokenLiteral not 5. got: {}", exp.token_literal())
        };
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) {x}".to_string();
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
                "program.statements doesn't contain {} statements. Got {}",
                1,
                program.statements.len()
            )
        };

        let stmt = match &program.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!(
                "program.statements[0] is not ExpressionStatement. got: {:?}",
                other
            ),
        };

        let exp = match &stmt.expression {
            EXPRESSION::IF(exp) => exp,
            _ => panic!("program.statement.expression is not of type ifExpression"),
        };

        test_infix_expression(
            &exp.condition,
            "x".to_string(),
            "<".to_string(),
            "y".to_string(),
        );

        if exp.consequence.statements.len() != 1 {
            panic!(
                "consequence is not 1 statements. got {}",
                exp.consequence.statements.len()
            )
        };

        let consequence = match &exp.consequence.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!(
                "program.statements[0] is not ExpressionStatement. got: {:?}",
                other
            ),
        };

        test_identifier(&consequence.expression, "x".to_string());

        if let Some(e) = &exp.alternative {
            panic!("exp.alternative.statements was not none. Got {:?}", e)
        };
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) {x} else {y}".to_string();
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
                "program.statements doesn't contain {} statements. Got {}",
                1,
                program.statements.len()
            )
        };

        let stmt = match &program.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!(
                "program.statements[0] is not ExpressionStatement. got: {:?}",
                other
            ),
        };

        let exp = match &stmt.expression {
            EXPRESSION::IF(exp) => exp,
            _ => panic!("program.statement.expression is not of type ifExpression"),
        };

        test_infix_expression(
            &exp.condition,
            "x".to_string(),
            "<".to_string(),
            "y".to_string(),
        );

        if exp.consequence.statements.len() != 1 {
            panic!(
                "consequence is not 1 statements. got {}",
                exp.consequence.statements.len()
            )
        };

        let consequence = match &exp.consequence.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!(
                "consequence.statements[0] is not ExpressionStatement. got: {:?}",
                other
            ),
        };

        test_identifier(&consequence.expression, "x".to_string());

        match &exp.alternative {
            Some(obj) => {
                if obj.statements.len() != 1 {
                    panic!(
                        "alternative is not 1 statements. Got {}",
                        obj.statements.len()
                    )
                }

                let alt = match &obj.statements[0] {
                    Statement::EXPRESSIONSTATEMENT(s) => s,
                    other => panic!(
                        "alternative.statements[0] is not ExpressionStatement. got: {:?}",
                        other
                    ),
                };

                test_identifier(&alt.expression, "y".to_string());
            }
            None => panic!("exp.alternative is none"),
        }
    }

    #[test]
    fn test_fn_literal_parsing() {
        let input = "fn (x, y) { x + y; }".to_string();
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
                "program.statements doesn't contain {} statements. Got {}",
                1,
                program.statements.len()
            )
        };

        let stmt = match &program.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!(
                "program.statements[0] is not ExpressionStatement. got: {:?}",
                other
            ),
        };

        let exp = match &stmt.expression {
            EXPRESSION::FN(exp) => exp,
            _ => panic!("program.statement.expression is not of type fn_literal"),
        };

        if exp.parameters.len() != 2 {
            panic!(
                "fn literal params wrong. Want 2, got {}",
                exp.parameters.len()
            )
        }

        test_identifier(
            &EXPRESSION::IDENTIFIER(exp.parameters[0].clone()),
            "x".to_string(),
        );
        test_identifier(
            &EXPRESSION::IDENTIFIER(exp.parameters[1].clone()),
            "y".to_string(),
        );

        if exp.body.statements.len() != 1 {
            panic!(
                "fn.body.statements has not 1 statement. Got {}",
                exp.body.statements.len()
            )
        }

        let stmt = match &exp.body.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!(
                "fn.body.statement[0] is not ExpressionStatement. Got {:?}",
                other
            ),
        };

        test_infix_expression(
            &stmt.expression,
            "x".to_string(),
            "+".to_string(),
            "y".to_string(),
        )
    }

    #[test]
    fn test_fn_params_parsing() {
        struct Test {
            input: String,
            expected: Vec<String>,
        }

        let tests = vec![
            Test {
                input: "fn() {};".to_string(),
                expected: vec![],
            },
            Test {
                input: "fn(x) {};".to_string(),
                expected: vec!["x".to_string()],
            },
            Test {
                input: "fn(x, y, z) {};".to_string(),
                expected: vec!["x".to_string(), "y".to_string(), "z".to_string()],
            },
        ];

        for test in tests {
            let mut l = Lexer::new(test.input);
            let mut p = Parser::new(&mut l);

            let program = p.parse_program();
            check_parser_errors(&p);

            let program = match program {
                Some(p) => p,
                None => panic!("parse_program returned nil"),
            };

            let stmt = match &program.statements[0] {
                Statement::EXPRESSIONSTATEMENT(s) => s,
                other => panic!(
                    "fn.body.statement[0] is not ExpressionStatement. Got {:?}",
                    other
                ),
            };

            let fn_literal = match &stmt.expression {
                EXPRESSION::FN(s) => s,
                other => panic!("statement.exprssion is not fn_literal. Got {:?}", other),
            };

            if fn_literal.parameters.len() != test.expected.len() {
                panic!(
                    "length parameter wrong. want {}, got {}",
                    test.expected.len(),
                    fn_literal.parameters.len()
                )
            }

            for (i, param) in test.expected.into_iter().enumerate() {
                test_literal_expression(
                    &EXPRESSION::IDENTIFIER(fn_literal.parameters[i].clone()),
                    param,
                )
            }
        }
    }

    #[test]
    fn test_call_expression_parsing() {
        let input = "add(1, 2 * 3, 4 + 5)".to_string();
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        let program = match program {
            Some(p) => p,
            None => panic!("parse_program returned nil"),
        };

        let stmt = match &program.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!(
                "fn.body.statement[0] is not ExpressionStatement. Got {:?}",
                other
            ),
        };

        let call_expression = match &stmt.expression {
            EXPRESSION::CALL(s) => s,
            other => panic!("statement.exprssion is not fn_literal. Got {:?}", other),
        };

        test_identifier(&call_expression.function, "add".to_string());

        if call_expression.args.len() != 3 {
            panic!(
                "length parameter wrong. want 3, got {}",
                call_expression.args.len()
            )
        }

        test_literal_expression(&call_expression.args[0], "1".to_string());
        test_infix_expression(
            &call_expression.args[1],
            "2".to_string(),
            "*".to_string(),
            "3".to_string(),
        );
        test_infix_expression(
            &call_expression.args[2],
            "4".to_string(),
            "+".to_string(),
            "5".to_string(),
        );
    }

    #[test]
    fn test_call_expression_args_parsing() {
        struct Test {
            input: String,
            expected: Vec<String>,
        }

        let tests = vec![
            Test {
                input: "add();".to_string(),
                expected: vec![],
            },
            Test {
                input: "add(x);".to_string(),
                expected: vec!["x".to_string()],
            },
            Test {
                input: "add(x, y, z);".to_string(),
                expected: vec!["x".to_string(), "y".to_string(), "z".to_string()],
            },
        ];

        for test in tests {
            let mut l = Lexer::new(test.input);
            let mut p = Parser::new(&mut l);

            let program = p.parse_program();
            check_parser_errors(&p);

            let program = match program {
                Some(p) => p,
                None => panic!("parse_program returned nil"),
            };

            let stmt = match &program.statements[0] {
                Statement::EXPRESSIONSTATEMENT(s) => s,
                other => panic!(
                    "fn.body.statement[0] is not ExpressionStatement. Got {:?}",
                    other
                ),
            };

            let call_expression = match &stmt.expression {
                EXPRESSION::CALL(s) => s,
                other => panic!(
                    "statement.exprssion is not call_expression. Got {:?}",
                    other
                ),
            };

            if call_expression.args.len() != test.expected.len() {
                panic!(
                    "length parameter wrong. want {}, got {}",
                    test.expected.len(),
                    call_expression.args.len()
                )
            }

            for (i, param) in test.expected.into_iter().enumerate() {
                test_literal_expression(&call_expression.args[i], param)
            }
        }
    }

    #[test]
    fn test_string_literal() {
        let input = "\"Hello World\"".to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        let program = match program {
            Some(program) => program,
            None => panic!("Program not found"),
        };

        let stmt = match &program.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!("Statement is not of type expression. Got: {:?}", other),
        };

        let literal = match &stmt.expression {
            EXPRESSION::StringLiteral(s) => s,
            other => panic!("expression not StringLiteral. Got: {:?}", other),
        };

        if literal.value != String::from("Hello World") {
            panic!("Literal val not {}. Got: {}", "Hello World", literal.value);
        }
    }

    #[test]
    fn test_array_literal() {
        let input = "[1, 2 * 2, true]".to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        let program = match program {
            Some(program) => program,
            None => panic!("Program not found"),
        };

        let stmt = match &program.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!("Statement is not of type expression. Got: {:?}", other),
        };

        let arr = match &stmt.expression {
            EXPRESSION::ArrayLiteral(s) => s,
            other => panic!("expression not ArrayLiteral. Got: {:?}", other),
        };
        if arr.items.len() != 3 {
            panic!("expected array.items.len to be 3, got: {}", arr.items.len())
        }

        test_integer_literal(&arr.items[0], 1);
        test_infix_expression(&arr.items[1], 2.to_string(), "*".to_string(), 2.to_string());
        test_boolean_literal(&arr.items[2], "true".to_string());
    }

    #[test]
    fn test_parsing_index_expression() {
        let input = "myArray[1 + 1]".to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        let program = match program {
            Some(program) => program,
            None => panic!("Program not found"),
        };

        let stmt = match &program.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!("Statement is not of type expression. Got: {:?}", other),
        };

        let exp = match &stmt.expression {
            EXPRESSION::IndexExpression(s) => s,
            other => panic!("expression not ArrayLiteral. Got: {:?}", other),
        };

        test_identifier(&exp.left, "myArray".to_string());
        test_infix_expression(
            &exp.index,
            "1".to_string(),
            "+".to_string(),
            "1".to_string(),
        );
    }

    #[test]
    fn test_parsing_hash_literal_string_keys() {
        let input = "{\"one\": 1, \"two\": 2}".to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        let program = match program {
            Some(program) => program,
            None => panic!("Program not found"),
        };

        let stmt = match &program.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!("Statement is not of type expression. Got: {:?}", other),
        };

        let hash = match &stmt.expression {
            EXPRESSION::HashLiteral(s) => s,
            other => panic!("expression not HashLiteral. Got: {:?}", other),
        };

        if hash.pairs.len() != 2 {
            panic!("Hash.pairs has wrong length. Got:{}", hash.pairs.len())
        }

        let mut expected = HashMap::new();
        expected.insert("one", 1);
        expected.insert("two", 2);

        for (k, v) in &hash.pairs {
            let key = match k {
                EXPRESSION::StringLiteral(str) => str.value.clone(),
                other => panic!("Expected key to be of type string. Got: {:?}", other),
            };

            let expected_val = expected.get(key.as_str()).unwrap();

            test_integer_literal(&v, *expected_val as i64);
        }
    }
    #[test]
    fn test_parsing_hash_literal_integer_keys() {
        let input = "{1: 1, 2: 2}".to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        let program = match program {
            Some(program) => program,
            None => panic!("Program not found"),
        };

        let stmt = match &program.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!("Statement is not of type expression. Got: {:?}", other),
        };

        let hash = match &stmt.expression {
            EXPRESSION::HashLiteral(s) => s,
            other => panic!("expression not HashLiteral. Got: {:?}", other),
        };

        if hash.pairs.len() != 2 {
            panic!("Hash.pairs has wrong length. Got:{}", hash.pairs.len())
        }

        let mut expected = HashMap::new();
        expected.insert("1", 1);
        expected.insert("2", 2);

        for (k, v) in &hash.pairs {
            let key = match k {
                EXPRESSION::INTEGER(int) => int.value.to_string(),
                other => panic!("Expected key to be of type integer. Got: {:?}", other),
            };

            let expected_val = expected.get(key.as_str()).unwrap();

            test_integer_literal(&v, *expected_val as i64);
        }
    }
    #[test]
    fn test_parsing_hash_literal_boolean_keys() {
        let input = "{true: 1, false: 0}".to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        let program = match program {
            Some(program) => program,
            None => panic!("Program not found"),
        };

        let stmt = match &program.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!("Statement is not of type expression. Got: {:?}", other),
        };

        let hash = match &stmt.expression {
            EXPRESSION::HashLiteral(s) => s,
            other => panic!("expression not HashLiteral. Got: {:?}", other),
        };

        if hash.pairs.len() != 2 {
            panic!("Hash.pairs has wrong length. Got:{}", hash.pairs.len())
        }

        let mut expected = HashMap::new();
        expected.insert("true", 1);
        expected.insert("false", 0);

        for (k, v) in &hash.pairs {
            let key = match k {
                EXPRESSION::BOOLEAN(bool) => bool.value.to_string(),
                other => panic!("Expected key to be of type boolean. Got: {:?}", other),
            };

            let expected_val = expected.get(key.as_str()).unwrap();

            test_integer_literal(&v, *expected_val as i64);
        }
    }
    #[test]
    fn test_parsing_hash_literal_with_exp() {
        let input = "{\"one\": 0 + 1, \"two\": 4 / 2}".to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        let program = match program {
            Some(program) => program,
            None => panic!("Program not found"),
        };

        let stmt = match &program.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!("Statement is not of type expression. Got: {:?}", other),
        };

        let hash = match &stmt.expression {
            EXPRESSION::HashLiteral(s) => s,
            other => panic!("expression not HashLiteral. Got: {:?}", other),
        };

        if hash.pairs.len() != 2 {
            panic!("Hash.pairs has wrong length. Got:{}", hash.pairs.len())
        }

        struct Expected {
            left: String,
            operator: String,
            right: String,
        }

        let mut expected = HashMap::new();
        expected.insert(
            "one",
            Expected {
                left: 0.to_string(),
                operator: "+".to_string(),
                right: 1.to_string(),
            },
        );
        expected.insert(
            "two",
            Expected {
                left: 4.to_string(),
                operator: "/".to_string(),
                right: 2.to_string(),
            },
        );

        for (k, v) in &hash.pairs {
            let key = match k {
                EXPRESSION::StringLiteral(str) => str.value.clone(),
                other => panic!("Expected key to be of type string. Got: {:?}", other),
            };

            let expected = expected.get(key.as_str()).unwrap();

            test_infix_expression(
                &v,
                expected.left.clone(),
                expected.operator.clone(),
                expected.right.clone(),
            );
        }
    }

    #[test]
    fn test_parsing_empty_hash_literal() {
        let input = "{}".to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        let program = match program {
            Some(program) => program,
            None => panic!("Program not found"),
        };

        let stmt = match &program.statements[0] {
            Statement::EXPRESSIONSTATEMENT(s) => s,
            other => panic!("Statement is not of type expression. Got: {:?}", other),
        };

        let hash = match &stmt.expression {
            EXPRESSION::HashLiteral(s) => s,
            other => panic!("expression not HashLiteral. Got: {:?}", other),
        };

        if hash.pairs.len() != 0 {
            panic!("Hash.pairs has wrong length. Got:{}", hash.pairs.len())
        }
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

    fn test_infix_expression(exp: &EXPRESSION, left: String, op: String, right: String) {
        match exp {
            EXPRESSION::INFIX(obj) => {
                test_literal_expression(&obj.left, left);

                if obj.operator != op {
                    panic!("exp.operator is not {}. Got {}", op, obj.operator);
                }

                test_literal_expression(&obj.right, right);
            }
            other => panic!("exp is not infix expression. Got {:?}", other),
        }
    }

    fn test_literal_expression(exp: &EXPRESSION, expected: String) {
        match exp {
            EXPRESSION::IDENTIFIER(_) => test_identifier(exp, expected),
            EXPRESSION::INTEGER(_) => test_integer_literal(exp, expected.parse().unwrap()),
            EXPRESSION::BOOLEAN(_) => test_boolean_literal(exp, expected),
            other => panic!("type of exp not handled. Got {:?}", other),
        }
    }

    fn test_boolean_literal(exp: &EXPRESSION, expected: String) {
        match exp {
            EXPRESSION::BOOLEAN(obj) => {
                if obj.value != expected.parse().unwrap() {
                    panic!(
                        "booleanExpression value not {}. Got {}",
                        expected, obj.value
                    )
                }

                if obj.token_literal() != expected {
                    panic!(
                        "booleanExpression tokenLiteral not {}. Got {}",
                        expected,
                        obj.token_literal()
                    )
                }
            }
            other => panic!("exp not BooleanExpression. Got {:?}", other),
        }
    }

    fn test_identifier(exp: &EXPRESSION, val: String) {
        match exp {
            EXPRESSION::IDENTIFIER(ident) => {
                if ident.value != val {
                    panic!("ident.value not {}. Got {}", val, ident.value)
                }

                if ident.token_literal() != val {
                    panic!(
                        "ident.token_literal not {}. Got {}",
                        val,
                        ident.token_literal()
                    )
                }
            }
            other => panic!("exp not idnetifier. Got {:?}", other),
        }
    }
}
