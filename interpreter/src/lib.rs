use std::path::Path;
use std::fs;

use crate::parser::ast::Stmt;
use crate::parser::parser::Parser;

pub mod parser;

pub struct SKInterpreter;

impl SKInterpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, source: &Path) -> Result<String, String> {
        let raw = fs::read_to_string(source).map_err(|e| e.to_string())?;

        let tokens = parser::lexer::tokenize(raw)?;

        let mut parser = Parser::new(tokens); // Why did I do this?
        let program = parser.parse()?;     // just to do this later

        // debug
        self.debug_ast(&program);

        Ok("output".to_string())
    }

    fn debug_ast(&self, program: &Vec<Stmt>) {
        println!("--- Abstract Syntax Tree ---");
        for (i, stmt) in program.iter().enumerate() {
            println!("Statement {}:", i);
            println!("{:#?}", stmt);
            println!("-----------------------");
        }
    }
}