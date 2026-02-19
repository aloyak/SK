use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::parser::lexer::TokenSpan;
use crate::core::error::Error;
use crate::core::value::Value;

pub fn register(env: &mut Environment) {
    env.define("split".into(), Value::NativeFn(split));
    env.define("trim".into(), Value::NativeFn(trim));
    env.define("replace".into(), Value::NativeFn(replace));
    env.define("toUpper".into(), Value::NativeFn(to_upper));
    env.define("toLower".into(), Value::NativeFn(to_lower));

}

pub fn split(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.is_empty() || args.len() < 1 {
        return Err(eval.error(span, "split() expects 1 string"));
    }

    let string = match &args[0] {
        Value::String(s) => s,
        _ => return Err(eval.error(span, "split() expects a string as the first argument")),
    };

    let delimiter = match args.get(1) {
        Some(Value::String(s)) => s,
        None => " ", // Default to splitting on whitespace
        _ => return Err(eval.error(span, "split() expects an optional string as the second argument")),
    };

    let parts: Vec<Value> = string.split(delimiter).map(|s| Value::String(s.to_string())).collect();
    Ok(Value::Array(parts))
}

pub fn replace(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.len() < 3 {
        return Err(eval.error(span, "replace() expects 3 arguments: string, target, replacement"));
    }
    
    let string = match &args[0] {
        Value::String(s) => s,
        _ => return Err(eval.error(span, "replace() expects a string as the first argument")),
    };

    let target = match &args[1] {
        Value::String(s) => s,
        _ => return Err(eval.error(span, "replace() expects a string as the second argument")),
    };

    let replacement = match &args[2] {
        Value::String(s) => s,
        _ => return Err(eval.error(span, "replace() expects a string as the third argument")),
    };

    let result = string.replace(target, replacement);
    Ok(Value::String(result))
}

pub fn to_upper(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.is_empty() || args.len() < 1 {
        return Err(eval.error(span, "toUpper() expects 1 string"));
    }

    let string = match &args[0] {
        Value::String(s) => s,
        _ => return Err(eval.error(span, "toUpper() expects a string"))
    };

    Ok(Value::String(string.to_uppercase()))
}

pub fn to_lower(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.is_empty() || args.len() < 1 {
        return Err(eval.error(span, "toLower() expects 1 string"));
    }

    let string = match &args[0] {
        Value::String(s) => s,
        _ => return Err(eval.error(span, "toLower() expects a string"))
    };

    Ok(Value::String(string.to_lowercase()))
}

pub fn trim(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.is_empty() || args.len() < 1 {
        return Err(eval.error(span, "trim() expects 1 string"));
    }

    let string = match &args[0] {
        Value::String(s) => s,
        _ => return Err(eval.error(span, "trim() expects a string"))
    };

    Ok(Value::String(string.trim().to_string()))
}
