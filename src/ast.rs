use crate::token::Token;

use std::any::Any;

pub trait Node: Any {
    fn token_litteral(&self) -> String;
    fn to_string(&self) -> String;
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

    fn to_string(&self) -> String {
        let mut buf = String::new();

        for statement in &self.statements {
            buf += &statement.to_string();
        }

        return buf;
    }
}

// ======================================================
// Expression Statement Node
pub struct ExpressionStatement {
    pub token: Token, // the first token of the statement
    pub expression: Box<dyn Expression>,
}

impl Node for ExpressionStatement {
    fn token_litteral(&self) -> String {
        return self.token.litteral();
    }

    fn to_string(&self) -> String {
        return self.expression.to_string();
    }
}
impl Statement for ExpressionStatement {}

// ======================================================
// Let Statement Node
pub struct LetStatement {
    pub token: Token,
    pub identifier: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_litteral(&self) -> String {
        return self.token.litteral();
    }

    fn to_string(&self) -> String {
        return format!(
            "{} {} = {};",
            &self.token.litteral(),
            &self.identifier.to_string(),
            &self.value.to_string()
        );
    }
}
impl Statement for LetStatement {}

// ======================================================
// Return Statement Node
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Box<dyn Expression>,
}

impl Node for ReturnStatement {
    fn token_litteral(&self) -> String {
        return self.token.litteral();
    }

    fn to_string(&self) -> String {
        return format!(
            "{} {};",
            self.token.litteral(),
            self.return_value.to_string()
        );
    }
}

impl Statement for ReturnStatement {}

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

    fn to_string(&self) -> String {
        return self.value.clone();
    }
}
impl Expression for Identifier {}

#[cfg(test)]
mod tests {
    use super::{Identifier, LetStatement, Node, Program, Statement, Token};

    #[test]
    fn test_string() {
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        statements.push(Box::new(LetStatement {
            token: Token::Let,
            identifier: Identifier {
                token: Token::Ident("my_var".to_string()),
                value: "my_var".to_string(),
            },
            value: Box::new(Identifier {
                token: Token::Ident("another_var".to_string()),
                value: "another_var".to_string(),
            }),
        }));

        let program = Program { statements };

        assert_eq!(program.to_string(), "let my_var = another_var;")
    }
}
