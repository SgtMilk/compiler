use crate::token::Token;

#[derive(Default)]
pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    /// Creates a lexer instance from an input string
    pub fn new(input_string: &str) -> Self {
        let mut lexer = Self {
            input: input_string.as_bytes().to_vec(),
            ..Default::default()
        };

        lexer.read_char();

        return lexer;
    }

    /// Returns the next token in the string
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            0 => Token::Eof,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let identifier = self.read_identifier();

                // early return to not re-read_char
                return match identifier.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "true" => Token::True,
                    "false" => Token::False,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    _ => Token::Ident(identifier),
                };
            }
            b'0'..=b'9' => return Token::Int(self.read_number()), // early return to not re-read_char
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b',' => Token::Comma,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'<' => Token::LesserThan,
            b'>' => Token::GreaterThan,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            _ => {
                let var = self.ch as char;
                unreachable!(
                "You are a bad monkey for trying to fit other characters like `{var}`. Shame on yourself. 🙊"
            )
            }
        };

        self.read_char();
        return token;
    }

    /// Reads the next character in the input and updates the lexer
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    /// peaks at the next chasracter without advancing the lexer
    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }

    /// reads the whole identifier
    /// WARN: you have to skip reading a char after this (or go back a position)
    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;

        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char()
        }

        String::from_utf8_lossy(&self.input[start_pos..self.position]).to_string()
    }

    /// reads the whole number
    /// WARN: you have to skip reading a char after this (or go back a position)
    fn read_number(&mut self) -> String {
        let start_pos = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[start_pos..self.position]).to_string()
    }

    /// skips whitespace
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

// ==================================================================
// HELPERS

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

    #[test]
    fn token_recognition_single_char() {
        let test_string = ")(){}{}+,;;=";

        let expected_output = [
            Token::RParen,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::LBrace,
            Token::RBrace,
            Token::Plus,
            Token::Comma,
            Token::Semicolon,
            Token::Semicolon,
            Token::Assign,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(&test_string);

        for (i, expected) in expected_output.iter().enumerate() {
            let generated = lexer.next_token();
            assert_eq!(
                generated, *expected,
                "Wrong token generated at character position `{i}` (expected: `{expected}`, generated: `{generated}`)"
            );
        }
    }

    #[test]
    fn token_recognition_code() {
        let test_string = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;";

        let expected_output = [
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Int("5".to_string()),
            Token::LesserThan,
            Token::Int("10".to_string()),
            Token::GreaterThan,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int("5".to_string()),
            Token::LesserThan,
            Token::Int("10".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int("10".to_string()),
            Token::Equal,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Int("10".to_string()),
            Token::NotEqual,
            Token::Int("9".to_string()),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(&test_string);

        for (i, expected) in expected_output.iter().enumerate() {
            let generated = lexer.next_token();
            assert_eq!(
                generated, *expected,
                "Wrong token generated at character position `{i}` (expected: `{expected}`, generated: `{generated}`)"
            );
        }
    }
}
