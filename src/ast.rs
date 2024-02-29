use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}
pub struct Statement {}
impl Statement {
    pub fn _statement_node() {}
}
impl Node for Statement {
    fn token_literal(&self) -> String {
        "Statement".to_string()
    }
}
#[derive(Debug)]
pub struct Expression {}
impl Expression {
    pub fn _expression_node() {}
}
impl Node for Expression {
    fn token_literal(&self) -> String {
        "Expression".to_string()
    }
}
pub struct Program {
    pub statements: Vec<StatementTypes>,
}
fn something(p: &Program) -> String {
    match p.statements.len() {
        0 => "".to_string(),
        _ => match &p.statements[0] {
            StatementTypes::LETSTATEMENT(obj) => obj.token_literal(),
        },
    }
}
#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
impl Identifier {
    fn _expression_node() {}
}
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}
impl LetStatement {
    fn _expression_node() {}
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
#[derive(Debug)]
pub enum StatementTypes {
    LETSTATEMENT(LetStatement),
}
