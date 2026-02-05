use crate::core::value::{Value, SKBool};
use crate::evaluator::eval::Evaluator;
use crate::core::error::Error;
use crate::parser::lexer::TokenSpan;
use std::io::{self, Write};

fn err(token: TokenSpan, msg: String) -> Error {
    Error {
        token,
        message: msg
    }
}

pub fn print(args: Vec<Value>, _span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    for arg in args {
        print!("{} ", arg);
    }
    println!();
    Ok(Value::None)
}

pub fn resolve(args: Vec<Value>, span: TokenSpan,  eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Symbolic { expression, .. }) => {
            eval.evaluate_expression(*expression.clone())
        }
        Some(Value::Unknown) => Ok(Value::None),
        Some(val) => Ok(val.clone()),
        None => Err(err(span, "resolve() expects 1 argument".to_string())),
    }
}

pub fn input(args: Vec<Value>, span: TokenSpan,  _: &mut Evaluator) -> Result<Value, Error> {
    if let Some(msg) = args.first() {
        print!("{}", msg);
        io::stdout().flush().map_err(|e| err(span.clone(), e.to_string()))?;
    }
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).map_err(|e| err(span, e.to_string()))?;
    Ok(Value::String(buffer.trim().to_string()))
}

pub fn num(args: Vec<Value>, span: TokenSpan,  _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::String(s)) => s.parse::<f64>().map(Value::Number).map_err(|_| err(span, "Cannot convert string to number".to_string())),
        Some(Value::Number(n)) => Ok(Value::Number(*n)),
        _ => Err(err(span, "num() expects a string or number".to_string())),
    }
}

pub fn str(args: Vec<Value>, _span: TokenSpan,  _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(val) => Ok(Value::String(val.to_string())),
        None => Ok(Value::String("".to_string())),
    }
}

pub fn certain(args: Vec<Value>, span: TokenSpan,  _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Bool(SKBool::True)) => Ok(Value::Bool(SKBool::True)),
        Some(_) => Ok(Value::Bool(SKBool::False)),
        None => Err(err(span, "certain() expects 1 argument".to_string())),
    }
}

pub fn impossible(args: Vec<Value>, span: TokenSpan,  _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Bool(SKBool::False)) => Ok(Value::Bool(SKBool::True)),
        Some(_) => Ok(Value::Bool(SKBool::False)),
        None => Err(err(span, "impossible() expects 1 argument".to_string())),
    }
}

pub fn possible(args: Vec<Value>, _span: TokenSpan,  _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Bool(SKBool::True)) | Some(Value::Bool(SKBool::Partial)) => Ok(Value::Bool(SKBool::True)),
        Some(Value::Bool(SKBool::False)) => Ok(Value::Bool(SKBool::False)),
        _ => Ok(Value::Bool(SKBool::True)), 
    }
}

pub fn known(args: Vec<Value>, span: TokenSpan,  _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Unknown) | Some(Value::Symbolic { .. }) => Ok(Value::Bool(SKBool::False)),
        Some(_) => Ok(Value::Bool(SKBool::True)),
        None => Err(err(span, "known() expects 1 argument".to_string())),
    }
}

pub fn kind(args: Vec<Value>, _span: TokenSpan,  _: &mut Evaluator) -> Result<Value, Error> {
    let t = match args.first() {
        Some(Value::Number(_)) => "number",
        Some(Value::String(_)) => "string",
        Some(Value::Bool(_)) => "bool",
        Some(Value::Interval(..)) => "interval",
        Some(Value::Unknown) => "unknown",
        Some(Value::Symbolic { is_quiet: true, .. }) => "quiet",
        Some(Value::Symbolic { .. }) => "symbolic",
        Some(Value::NativeFn(_)) => "native function",
        Some(Value::Function(_)) => "function",
        Some(Value::Module(_)) => "module",
        Some(Value::None) | None => "none",
    };
    Ok(Value::String(t.to_string()))
}

pub fn intersect(args: Vec<Value>, span: TokenSpan,  _: &mut Evaluator) -> Result<Value, Error> {
    if args.len() != 2 { return Err(err(span, "intersect() expects 2 arguments".to_string())); }
    match (&args[0], &args[1]) {
        (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
            let low = min1.max(*min2);
            let high = max1.min(*max2);
            if low <= high { Ok(Value::Interval(low, high)) } else { Ok(Value::None) }
        }
        _ => Err(err(span, "intersect() requires two intervals".to_string())),
    }
}

pub fn union(args: Vec<Value>, span: TokenSpan,  _: &mut Evaluator) -> Result<Value, Error> {
    if args.len() != 2 { return Err(err(span, "union() expects 2 arguments".to_string())); }
    match (&args[0], &args[1]) {
        (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
            Ok(Value::Interval(min1.min(*min2), max1.max(*max2)))
        }
        (Value::Number(n), Value::Interval(min, max)) | (Value::Interval(min, max), Value::Number(n)) => {
            Ok(Value::Interval(min.min(*n), max.max(*n)))
        }
        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Interval(n1.min(*n2), n1.max(*n2))),
        _ => Err(err(span, "union() expects intervals or numbers".to_string())),
    }
}

pub fn mid(args: Vec<Value>, span: TokenSpan,  _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Interval(min, max)) => Ok(Value::Number((min + max) / 2.0)),
        _ => Err(err(span, "mid() expects an interval".to_string())),
    }
}

pub fn width(args: Vec<Value>, span: TokenSpan,  _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Interval(min, max)) => Ok(Value::Number(max - min)),
        _ => Err(err(span, "width() expects an interval".to_string())),
    }
}