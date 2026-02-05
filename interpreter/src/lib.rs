use std::path::Path;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

pub mod core;
pub mod parser;
pub mod evaluator;
pub mod libs;

use crate::parser::lexer::{Lexer, Token, TokenSpan};
use crate::parser::parser::Parser;
use crate::evaluator::eval::Evaluator;
use crate::evaluator::env::Environment;
use crate::core::value::Value;
use crate::core::error::Error;

pub struct SKInterpreter {
    env: Rc<RefCell<Environment>>,
}

impl SKInterpreter {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn execute(&mut self, source: &Path) -> Result<Value, Error> {
        let raw = fs::read_to_string(source).map_err(|e| Error {
            token: TokenSpan {
                token: Token::None,
                line: 0,
                column: 0,
            },
            message: format!("{}", e),
        })?;

        self.execute_string(raw)
    }

    pub fn execute_string(&mut self, source: String) -> Result<Value, Error> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().map_err(|msg| Error {
            token: TokenSpan {
                token: Token::None,
                line: 0,
                column: 0,
            },
            message: msg,
        })?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().map_err(|msg| Error {
            token: TokenSpan {
                token: Token::None,
                line: 0,
                column: 0,
            },
            message: msg.to_string(),
        })?;

        let mut evaluator = Evaluator::new(self.env.clone());
        evaluator.evaluate(ast)
    }
}