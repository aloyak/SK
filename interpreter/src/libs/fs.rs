use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::parser::lexer::TokenSpan;
use crate::core::error::Error;
use crate::core::value::Value;

pub fn register(env: &mut Environment) {
    env.define("read".into(), Value::NativeFn(read));
}

fn err(token: TokenSpan, msg: String) -> Error {
    Error {
        token,
        message: msg
    }
}

pub fn read(_args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    Err(err(span, "Not implemented!".to_string()))
}

pub fn write(_args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    Err(err(span, "Not implemented!".to_string()))
}