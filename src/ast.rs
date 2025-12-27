use crate::token::Token;

pub trait Node {
    fn token_litteral(&self) -> String;
}

/// statement nodes (doesn't produce a value)
pub trait Statement: Node {}

// expression nodes (produces a value -- ex: add(5, 5) or 5)
pub trait Expression: Node {}

// ======================================================
// Root Program Node
struct Program {
    statements: Vec<Box<dyn Statement>>,
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
struct LetStatement<'a> {
    token: Token,
    name: &'a Identifier,
    value: Box<dyn Expression>,
}

impl<'a> Node for LetStatement<'a> {
    fn token_litteral(&self) -> String {
        return "let".to_string();
    }
}
impl<'a> Statement for LetStatement<'a> {}

// ======================================================
// Identifier Node
struct Identifier {
    token: Token,
    value: String,
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
