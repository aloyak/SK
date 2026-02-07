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
use crate::core::error::{Error, ErrorReporter, Warning};

pub struct SKInterpreter {
    env: Rc<RefCell<Environment>>,
    reporter: Rc<RefCell<ErrorReporter>>,
}

impl SKInterpreter {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new())),
            reporter: Rc::new(RefCell::new(ErrorReporter::new())),
        }
    }

    pub fn take_warnings(&mut self) -> Vec<Warning> {
        self.reporter.borrow_mut().take_warnings()
    }

    pub fn execute(&mut self, source: &Path) -> Result<Value, Error> {
        let raw = fs::read_to_string(source).map_err(|e| {
            self.reporter
                .borrow_mut()
                .error(TokenSpan {
                    token: Token::None,
                    line: 0,
                    column: 0,
                }, format!("{}", e))
        })?;

        self.execute_named(source.display().to_string(), raw)
    }

    pub fn execute_string(&mut self, source: String) -> Result<Value, Error> {
        self.execute_named("<repl>".to_string(), source)
    }

    fn execute_named(&mut self, name: String, source: String) -> Result<Value, Error> {
        let previous = self.reporter.borrow_mut().set_source(name, source.clone());

        let result = (|| {
            let mut lexer = Lexer::new(source, self.reporter.clone());
            let tokens = lexer.tokenize()?;

            let mut parser = Parser::new(tokens, self.reporter.clone());
            let ast = parser.parse()?;

            let mut evaluator = Evaluator::new(self.env.clone(), self.reporter.clone());
            evaluator.evaluate(ast)
        })();

        self.reporter.borrow_mut().restore_source(previous);

        result
    }
}