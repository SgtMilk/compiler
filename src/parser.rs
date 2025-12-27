use crate::lexer::Lexer;
use crate::token::Token;

struct Parser<'a> {
    lexer: &'a mut Lexer,

    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
        };

        parser.next_token();
        parser.next_token();

        return parser;
    }

    fn next_token(&mut self) {
        // peek_token goes into cur_token and peek_token gets a new value
        self.cur_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }
}
