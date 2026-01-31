use std::path::Path;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

pub mod core;
pub mod parser;
pub mod evaluator;

use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::evaluator::eval::Evaluator;
use crate::evaluator::env::Environment;
use crate::core::value::Value;
use crate::parser::ast::Stmt;

pub struct SKInterpreter {
    env: Rc<RefCell<Environment>>,
}

impl SKInterpreter {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn execute(&mut self, source: &Path) -> Result<Value, String> {
        let raw = fs::read_to_string(source).map_err(|e| e.to_string())?;

        let mut lexer = Lexer::new(raw);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        //self._debug_ast(&ast);

        let mut evaluator = Evaluator::new(self.env.clone());
        evaluator.evaluate(ast)
    }

    pub fn execute_string(&mut self, source: String) -> Result<Value, String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        let mut evaluator = Evaluator::new(self.env.clone());
        evaluator.evaluate(ast)
    }

    fn _debug_ast(&self, program: &Vec<Stmt>) {
        println!("--- Abstract Syntax Tree ---");
        for (i, stmt) in program.iter().enumerate() {
            println!("Statement {}:", i);
            println!("{:#?}", stmt);
            println!("-----------------------");
        }
    }
}