pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

use crate::lexer::Lexer;
use crate::token::Token;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

const PROMPT: &str = ">> ";

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    println!("Hello! This is the Monkey Programming Language!");
    println!("Feel free to type in commands.");

    loop {
        let readline = rl.readline(PROMPT);
        match readline {
            Ok(line) => {
                let mut lexer = Lexer::new(line.as_str());

                let mut token = lexer.next_token();
                while token != Token::Eof {
                    print!("{token}, ");
                    token = lexer.next_token();
                }
                print!("{token}\n");
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
