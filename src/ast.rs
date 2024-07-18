use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub struct Program {
    pub statements: Vec<StatementTypes>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        match self.statements.len() {
            0 => "".to_string(),
            _ => self.statements[0].token_literal(),
        }
    }
    fn string(&self) -> String {
        let mut str = String::new();

        for stmt in self.statements.iter() {
            str.push_str(&stmt.string());
        }

        str
    }
}

#[derive(Debug)]
pub enum StatementTypes {
    LETSTATEMENT(LetStatement),
    RETURNSTATEMENT(ReturnStatement),
    EXPRESSIONSTATEMENT(ExpressionStatement),
}

impl Node for StatementTypes {
    fn token_literal(&self) -> String {
        match self {
            Self::LETSTATEMENT(stmt) => stmt.token_literal(),
            Self::RETURNSTATEMENT(stmt) => stmt.token_literal(),
            Self::EXPRESSIONSTATEMENT(stmt) => stmt.token_literal(),
        }
    }
    fn string(&self) -> String {
        match self {
            Self::LETSTATEMENT(stmt) => stmt.string(),
            Self::RETURNSTATEMENT(stmt) => stmt.string(),
            Self::EXPRESSIONSTATEMENT(stmt) => stmt.string(),
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: EXPRESSION,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut str = String::new();

        str = str + &self.token_literal() + " ";
        str = str + &self.name.string() + " = ";
        str = str + &self.value.string() + ";";

        str
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: EXPRESSION,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut str = String::new();

        str = str + &self.token_literal() + " ";
        str = str + ";";

        str
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: EXPRESSION,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.expression.string()
    }
}

#[derive(Debug)]
pub enum EXPRESSION {
    IDENTIFIER(Identifier),
    INTEGER(IntegerLiteral),
    PREFIX(PrefixExpression),
    INFIX(InfixExpression),
}
impl Node for EXPRESSION {
    fn token_literal(&self) -> String {
        match self {
            EXPRESSION::IDENTIFIER(obj) => obj.token_literal(),
            EXPRESSION::INTEGER(obj) => obj.token_literal(),
            EXPRESSION::PREFIX(obj) => obj.token_literal(),
            EXPRESSION::INFIX(obj) => obj.token_literal(),
        }
    }
    fn string(&self) -> String {
        match self {
            EXPRESSION::IDENTIFIER(obj) => obj.string(),
            EXPRESSION::INTEGER(obj) => obj.string(),
            EXPRESSION::PREFIX(obj) => obj.string(),
            EXPRESSION::INFIX(obj) => obj.string(),
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<EXPRESSION>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub operator: String,
    pub left: Box<EXPRESSION>,
    pub right: Box<EXPRESSION>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.string(),
            self.operator,
            self.right.string()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::EXPRESSION,
        token::{Token, TokenType},
    };

    use super::{Identifier, LetStatement, Node, Program, StatementTypes};

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![StatementTypes::LETSTATEMENT(LetStatement {
                token: Token {
                    r#type: TokenType::LET,
                    literal: "let".to_string(),
                },
                name: Identifier {
                    token: Token {
                        r#type: TokenType::IDENT,
                        literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                },
                value: EXPRESSION::IDENTIFIER(Identifier {
                    token: Token {
                        r#type: TokenType::IDENT,
                        literal: "anotherVar".to_string(),
                    },
                    value: "anotherVar".to_string(),
                }),
            })],
        };
        println!("{}", program.string());

        if program.string() != String::from("let myVar = anotherVar;") {
            panic!("program.string() failed");
        }
    }
}
