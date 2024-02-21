use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}
struct Statement {}
impl Statement {
    pub fn _statement_node() {}
}
impl Node for Statement {
    fn token_literal(&self) -> String {
        "Statement".to_string()
    }
}
struct Expression {}
impl Expression {
    pub fn _expression_node() {}
}
impl Node for Expression {
    fn token_literal(&self) -> String {
        "Expression".to_string()
    }
}
struct Program {
    statements: Vec<Statement>,
}
fn something(p: &Program) -> String {
    match p.statements.len() {
        0 => "".to_string(),
        _ => p.statements[0].token_literal(),
    }
}
struct Identifier {
    token: Token,
    value: String,
}
impl Identifier {
    fn _expression_node() {}
}
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
struct LetStatement {
    token: Token,
    name: Identifier,
    value: Expression,
}
impl LetStatement {
    fn _expression_node() {}
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
