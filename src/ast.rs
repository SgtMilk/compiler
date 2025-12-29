use crate::token::Token;

use std::any::Any;

pub trait Node: Any {
    fn token_litteral(&self) -> String;
}

/// statement nodes (doesn't produce a value)
pub trait Statement: Node {}

// expression nodes (produces a value -- ex: add(5, 5) or 5)
pub trait Expression: Node {}

// ======================================================
// Root Program Node
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_litteral(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_litteral();
        } else {
            return "".to_string();
        }
    }
}

// ======================================================
// Let Statement Node
pub struct LetStatement {
    pub token: Token,
    pub identifier: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_litteral(&self) -> String {
        return "let".to_string();
    }
}
impl Statement for LetStatement {}

// ======================================================
// Identifier Node
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_litteral(&self) -> String {
        match &self.token {
            Token::Ident(value) => value.clone(),
            _ => {
                let tok = &self.token;
                unreachable!("It should be impossible to reach other token types in an expression identifier: {tok}")
            }
        }
    }
}
impl Expression for Identifier {}
