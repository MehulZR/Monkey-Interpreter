use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub struct Program {
    pub statements: Vec<Statement>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    LETSTATEMENT(LetStatement),
    RETURNSTATEMENT(ReturnStatement),
    EXPRESSIONSTATEMENT(ExpressionStatement),
}

impl Node for Statement {
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EXPRESSION {
    IDENTIFIER(Identifier),
    INTEGER(IntegerLiteral),
    PREFIX(PrefixExpression),
    INFIX(InfixExpression),
    BOOLEAN(BooleanExpression),
    IF(IfExpression),
    FN(FnExpression),
    CALL(CallExpression),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLitearl),
    IndexExpression(IndexExpression),
}
impl Node for EXPRESSION {
    fn token_literal(&self) -> String {
        match self {
            EXPRESSION::IDENTIFIER(obj) => obj.token_literal(),
            EXPRESSION::INTEGER(obj) => obj.token_literal(),
            EXPRESSION::PREFIX(obj) => obj.token_literal(),
            EXPRESSION::INFIX(obj) => obj.token_literal(),
            EXPRESSION::BOOLEAN(obj) => obj.token_literal(),
            EXPRESSION::IF(obj) => obj.token_literal(),
            EXPRESSION::FN(obj) => obj.token_literal(),
            EXPRESSION::CALL(obj) => obj.token_literal(),
            EXPRESSION::StringLiteral(obj) => obj.token_literal(),
            EXPRESSION::ArrayLiteral(obj) => obj.token_literal(),
            EXPRESSION::IndexExpression(obj) => obj.token_literal(),
        }
    }
    fn string(&self) -> String {
        match self {
            EXPRESSION::IDENTIFIER(obj) => obj.string(),
            EXPRESSION::INTEGER(obj) => obj.string(),
            EXPRESSION::PREFIX(obj) => obj.string(),
            EXPRESSION::INFIX(obj) => obj.string(),
            EXPRESSION::BOOLEAN(obj) => obj.string(),
            EXPRESSION::IF(obj) => obj.string(),
            EXPRESSION::FN(obj) => obj.string(),
            EXPRESSION::CALL(obj) => obj.string(),
            EXPRESSION::StringLiteral(obj) => obj.string(),
            EXPRESSION::ArrayLiteral(obj) => obj.string(),
            EXPRESSION::IndexExpression(obj) => obj.string(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BooleanExpression {
    pub token: Token,
    pub value: bool,
}

impl Node for BooleanExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<EXPRESSION>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut str = String::new();

        str.push_str("if");
        str.push_str(&self.condition.string());
        str.push_str(" ");
        str.push_str(&self.consequence.string());

        if let Some(alt) = &self.alternative {
            str.push_str("else ");
            str.push_str(&alt.string());
        };

        str
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut str = String::new();

        for stmt in self.statements.iter() {
            str.push_str(&stmt.string())
        }

        str
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FnExpression {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Node for FnExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut str = String::new();
        let params = self
            .parameters
            .iter()
            .map(|param| param.string())
            .collect::<Vec<String>>()
            .join(", ");

        str.push_str(&self.token_literal());
        str.push_str("(");
        str.push_str(&params);
        str.push_str(")");
        str.push_str(&self.body.string());

        str
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<EXPRESSION>,
    pub args: Vec<EXPRESSION>,
}
impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut str = String::new();
        let args = self
            .args
            .iter()
            .map(|arg| arg.string())
            .collect::<Vec<String>>()
            .join(", ");

        str.push_str(&self.function.string());
        str.push_str("(");
        str.push_str(&args);
        str.push_str(")");

        str
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl Node for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArrayLitearl {
    pub token: Token,
    pub items: Vec<EXPRESSION>,
}

impl Node for ArrayLitearl {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut str = String::new();

        let items = self
            .items
            .iter()
            .map(|item| item.string())
            .collect::<Vec<String>>()
            .join(", ");

        str.push_str("[");
        str.push_str(&items);
        str.push_str("]");

        str
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndexExpression {
    pub token: Token,
    pub left: Box<EXPRESSION>,
    pub index: Box<EXPRESSION>,
}

impl Node for IndexExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut str = String::new();

        str.push_str("(");
        str.push_str(&self.left.string());
        str.push_str("[");
        str.push_str(&self.index.string());
        str.push_str("])");

        str
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::EXPRESSION,
        token::{Token, TokenType},
    };

    use super::{Identifier, LetStatement, Node, Program, Statement};

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Statement::LETSTATEMENT(LetStatement {
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
