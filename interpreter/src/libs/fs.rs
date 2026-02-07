use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::parser::lexer::TokenSpan;
use crate::core::error::Error;
use crate::core::value::Value;

pub fn register(env: &mut Environment) {
    env.define("read".into(), Value::NativeFn(read));
}

pub fn read(_args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    Err(eval.error(span, "Not implemented!"))
}

pub fn write(_args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    Err(eval.error(span, "Not implemented!"))
}