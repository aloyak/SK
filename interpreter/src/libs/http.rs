use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::parser::lexer::TokenSpan;
use crate::core::error::Error;
use crate::core::value::Value;

// TODO!
fn ensure_unsafe(eval: &mut Evaluator, span: TokenSpan, fn_name: &str) -> Result<(), Error> {
    if eval.is_safe_mode() {
        return Err(eval.error(span, format!("{fn_name}() is disabled in --safe mode. Download the SK interpreter!")));
    }
    Ok(())
}

pub fn register(env: &mut Environment) {
    env.define("get".into(), Value::NativeFn(get));    
    env.define("post".into(), Value::NativeFn(post));
    env.define("request".into(), Value::NativeFn(request));
}

pub fn get(_args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    ensure_unsafe(eval, span.clone(), "http.get")?;

    Err(eval.error(span, "get() is not implemented yet!"))
}

pub fn post(_args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    ensure_unsafe(eval, span.clone(), "http.post")?;

    Err(eval.error(span, "post() is not implemented yet!"))
}

pub fn request(_args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    ensure_unsafe(eval, span.clone(), "http.request")?;

    Err(eval.error(span, "request() is not implemented yet!"))
}
