use crate::core::value::{Value, SKBool};
use crate::evaluator::eval::Evaluator;
use crate::core::error::Error;
use crate::parser::lexer::TokenSpan;
use std::io::{self, Write};

pub fn print(args: Vec<Value>, _span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    for arg in args {
        print!("{} ", arg);
    }
    println!();
    Ok(Value::None)
}

pub fn write(args: Vec<Value>, _span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    for arg in args {
        print!("{} ", arg);
    }
    Ok(Value::None)
}

pub fn resolve(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Symbolic { expression, .. }) => {
            eval.evaluate_expression(*expression.clone())
        }
        Some(Value::Unknown) => Ok(Value::None),
        Some(val) => Ok(val.clone()),
        None => Err(eval.error(span, "resolve() expects 1 argument")),
    }
}

pub fn input(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if let Some(msg) = args.first() {
        print!("{}", msg);
        io::stdout()
            .flush()
            .map_err(|e| eval.error(span.clone(), e.to_string()))?;
    }
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| eval.error(span, e.to_string()))?;
    Ok(Value::String(buffer.trim().to_string()))
}

pub fn num(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::String(s)) => s
            .parse::<f64>()
            .map(Value::Number)
            .map_err(|_| eval.error(span, "Cannot convert string to number")),
        Some(Value::Number(n)) => Ok(Value::Number(*n)),
        _ => Err(eval.error(span, "num() expects a string or number")),
    }
}

pub fn str(args: Vec<Value>, _span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(val) => Ok(Value::String(val.to_string())),
        None => Ok(Value::String("".to_string())),
    }
}

pub fn certain(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Bool(SKBool::True)) => Ok(Value::Bool(SKBool::True)),
        Some(_) => Ok(Value::Bool(SKBool::False)),
        None => Err(eval.error(span, "certain() expects 1 argument")),
    }
}

pub fn impossible(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Bool(SKBool::False)) => Ok(Value::Bool(SKBool::True)),
        Some(_) => Ok(Value::Bool(SKBool::False)),
        None => Err(eval.error(span, "impossible() expects 1 argument")),
    }
}

pub fn possible(args: Vec<Value>, _span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Bool(SKBool::True)) | Some(Value::Bool(SKBool::Partial)) => Ok(Value::Bool(SKBool::True)),
        Some(Value::Bool(SKBool::False)) => Ok(Value::Bool(SKBool::False)),
        _ => Ok(Value::Bool(SKBool::True)), 
    }
}

pub fn known(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Unknown) | Some(Value::Symbolic { .. }) => Ok(Value::Bool(SKBool::False)),
        Some(_) => Ok(Value::Bool(SKBool::True)),
        None => Err(eval.error(span, "known() expects 1 argument")),
    }
}

pub fn kind(args: Vec<Value>, _span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    let t = match args.first() {
        Some(Value::Number(_)) => "number",
        Some(Value::String(_)) => "string",
        Some(Value::Bool(_)) => "bool",
        Some(Value::Interval(..)) => "interval",
        Some(Value::Array(..)) => "array",
        Some(Value::Unknown) => "unknown",
        Some(Value::Quantity { .. }) => "quantity",
        Some(Value::Symbolic { is_quiet: true, .. }) => "quiet",
        Some(Value::Symbolic { .. }) => "symbolic",
        Some(Value::NativeFn(_)) => "native function",
        Some(Value::Function(_)) => "function",
        Some(Value::Module(_)) => "module",
        Some(Value::None) | None => "none",
    };
    Ok(Value::String(t.to_string()))
}

pub fn intersect(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(eval.error(span, "intersect() expects 2 arguments"));
    }
    match (&args[0], &args[1]) {
        (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
            let low = min1.max(*min2);
            let high = max1.min(*max2);
            if low <= high { Ok(Value::Interval(low, high)) } else { Ok(Value::None) }
        }
        _ => Err(eval.error(span, "intersect() requires two intervals")),
    }
}

pub fn union(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(eval.error(span, "union() expects 2 arguments"));
    }
    match (&args[0], &args[1]) {
        (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
            Ok(Value::Interval(min1.min(*min2), max1.max(*max2)))
        }
        (Value::Number(n), Value::Interval(min, max)) | (Value::Interval(min, max), Value::Number(n)) => {
            Ok(Value::Interval(min.min(*n), max.max(*n)))
        }
        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Interval(n1.min(*n2), n1.max(*n2))),
        _ => Err(eval.error(span, "union() expects intervals or numbers")),
    }
}

pub fn mid(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Interval(min, max)) => Ok(Value::Number((min + max) / 2.0)),
        _ => Err(eval.error(span, "mid() expects an interval")),
    }
}

pub fn width(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Interval(min, max)) => Ok(Value::Number(max - min)),
        _ => Err(eval.error(span, "width() expects an interval")),
    }
}