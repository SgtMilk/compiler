use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers and litterals
    Ident(String),
    Int(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LesserThan,
    GreaterThan,

    Equal,
    NotEqual,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Illegal => write!(f, "Illegal"),
            Token::Eof => write!(f, "Eof"),
            Token::Ident(x) => write!(f, "Ident({})", x),
            Token::Int(x) => write!(f, "Int({})", x),
            Token::Assign => write!(f, "Assign"),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Bang => write!(f, "Bang"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Slash => write!(f, "Slash"),
            Token::LesserThan => write!(f, "Lesserthan"),
            Token::GreaterThan => write!(f, "Greaterthan"),
            Token::Equal => write!(f, "Equal"),
            Token::NotEqual => write!(f, "NotEqual"),
            Token::Comma => write!(f, "Comma"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::LParen => write!(f, "Lparen"),
            Token::RParen => write!(f, "Rparen"),
            Token::LBrace => write!(f, "LSquirly"),
            Token::RBrace => write!(f, "RSquirly"),
            Token::Function => write!(f, "Function"),
            Token::Let => write!(f, "Let"),
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::Return => write!(f, "Return"),
        };
    }
}

impl Token {
    pub fn litteral(&self) -> String {
        let val = match self {
            Token::Illegal => "ILLEGAL",
            Token::Eof => "",
            Token::Ident(identifier) => &identifier,
            Token::Int(int_val) => &int_val,
            Token::Assign => "=",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Bang => "!",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::LesserThan => "<",
            Token::GreaterThan => ">",
            Token::Equal => "==",
            Token::NotEqual => "!=",
            Token::Comma => ",",
            Token::Semicolon => ",",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::Function => "function",
            Token::Let => "let",
            Token::True => "true",
            Token::False => "false",
            Token::If => "if",
            Token::Else => "else",
            Token::Return => "return",
        };

        return val.to_string();
    }
}
