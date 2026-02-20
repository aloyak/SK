use crate::core::value::{Value, SKBool};
use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::core::error::Error;
use crate::parser::lexer::TokenSpan;
use serde_json;

pub fn register(env: &mut Environment) {
    env.define("parse".into(), Value::NativeFn(parse));
    env.define("stringify".into(), Value::NativeFn(stringify));
}

pub fn parse(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(eval.error(span, "parse() expects 1 argument"));
    }

    let json_str = match &args[0] {
        Value::String(s) => s,
        _ => return Err(eval.error(span, "parse() expects a string argument")),
    };

    let json_value: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| eval.error(span.clone(), format!("JSON parse error: {}", e)))?;

    json_to_value(json_value, &span, eval)
}

pub fn stringify(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(eval.error(span, "stringify() expects 1 argument"));
    }

    let json_value = value_to_json(&args[0], &span, eval)?;
    let json_string = serde_json::to_string(&json_value)
        .map_err(|e| eval.error(span, format!("JSON stringify error: {}", e)))?;

    Ok(Value::String(json_string))
}

fn json_to_value(json: serde_json::Value, span: &TokenSpan, eval: &Evaluator) -> Result<Value, Error> {
    match json {
        serde_json::Value::Null => Ok(Value::None),
        serde_json::Value::Bool(b) => Ok(Value::Bool(if b { SKBool::True } else { SKBool::False })),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                Ok(Value::Number(f))
            } else {
                Err(eval.error(span.clone(), "Invalid JSON number"))
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(s)),
        serde_json::Value::Array(arr) => {
            let mut sk_arr = Vec::new();
            for item in arr {
                sk_arr.push(json_to_value(item, span, eval)?);
            }
            Ok(Value::Array(sk_arr))
        }
        serde_json::Value::Object(obj) => {
            // Convert JSON object to array of [key, value] pairs
            let mut pairs = Vec::new();
            for (key, value) in obj {
                let sk_value = json_to_value(value, span, eval)?;
                pairs.push(Value::Array(vec![Value::String(key), sk_value]));
            }
            Ok(Value::Array(pairs))
        }
    }
}

fn value_to_json(value: &Value, span: &TokenSpan, eval: &Evaluator) -> Result<serde_json::Value, Error> {
    match value {
        Value::None => Ok(serde_json::Value::Null),
        Value::Bool(SKBool::True) => Ok(serde_json::Value::Bool(true)),
        Value::Bool(SKBool::False) => Ok(serde_json::Value::Bool(false)),
        Value::Bool(SKBool::Partial) => Err(eval.error(span.clone(), "Cannot stringify 'partial' boolean to JSON")),
        Value::Number(n) => {
            if let Some(num) = serde_json::Number::from_f64(*n) {
                Ok(serde_json::Value::Number(num))
            } else {
                Err(eval.error(span.clone(), "Invalid number for JSON"))
            }
        }
        Value::String(s) => Ok(serde_json::Value::String(s.clone())),
        Value::Array(arr) => {
            let mut json_arr = Vec::new();
            for item in arr {
                json_arr.push(value_to_json(item, span, eval)?);
            }
            Ok(serde_json::Value::Array(json_arr))
        }
        _ => Err(eval.error(span.clone(), format!("Cannot stringify {} to JSON", value))),
    }
}
