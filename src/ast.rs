use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub trait Statement {
    fn statement_node() {}
}

pub trait ExpressionTrait {
    fn expression_node() {}
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
impl ExpressionTrait for Identifier {
    fn expression_node() {}
}

#[derive(Debug)]
pub struct Expression {}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut str = String::new();
        str = str + &self.token_literal() + " ";
        str = str + &self.name.string() + " = ";

        // if let Some(val) = self.value {
        //     str = str + val.string();
        // }

        str = str + ";";
        str
    }
}
impl Statement for LetStatement {
    fn statement_node() {}
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}
impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut str = String::new();
        str = str + &self.token_literal() + " ";

        // if let Some(val) = self.return_value {
        //     str = str + val;
        // }

        str = str + ";";
        str
    }
}
impl Statement for ReturnStatement {
    fn statement_node() {}
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}
impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        // if let Some(val) = self {
        //     return val;
        // }

        String::new()
    }
}
impl Statement for ExpressionStatement {
    fn statement_node() {}
}

#[derive(Debug)]
pub enum StatementTypes {
    LETSTATEMENT(LetStatement),
    RETURNSTATEMENT(ReturnStatement),
    EXPRESSIONSTATEMENT(ExpressionStatement),
}

pub struct Program {
    pub statements: Vec<StatementTypes>,
}
impl Node for Program {
    fn token_literal(&self) -> String {
        match self.statements.len() {
            0 => "".to_string(),
            _ => match &self.statements[0] {
                StatementTypes::LETSTATEMENT(obj) => obj.token_literal(),
                StatementTypes::RETURNSTATEMENT(obj) => obj.token_literal(),
                StatementTypes::EXPRESSIONSTATEMENT(obj) => obj.token_literal(),
            },
        }
    }
    fn string(&self) -> String {
        let mut str = String::new();
        for stmt in self.statements.iter() {
            match stmt {
                StatementTypes::LETSTATEMENT(s) => str = str + &s.string(),
                StatementTypes::RETURNSTATEMENT(s) => str = str + &s.string(),
                StatementTypes::EXPRESSIONSTATEMENT(s) => str = str + &s.string(),
            }
        }
        str
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::token::{Token, TokenType};
//
//     use super::{Identifier, LetStatement, Node, Program, StatementTypes};
//
// #[test]
// fn test_string() {
//     let program = Program {
//         statements: vec![StatementTypes::LETSTATEMENT(LetStatement {
//             token: Token {
//                 r#type: TokenType::LET,
//                 literal: "let".to_string(),
//             },
//             name: Identifier {
//                 token: Token {
//                     r#type: TokenType::IDENT,
//                     literal: "myVar".to_string(),
//                 },
//                 value: "myVar".to_string(),
//             },
//             value: Identifier {
//                 token: Token {
//                     r#type: TokenType::IDENT,
//                     literal: "anotherVar".to_string(),
//                 },
//                 value: "anotherVar".to_string(),
//             },
//         })],
//     };
//     if program.string() != String::from("let myVar = anotherVar;") {
//         panic!("program.string() failed");
//     }
// }
// }
