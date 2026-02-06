use crate::core::value::Value;
use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::core::error::Error;
use crate::parser::lexer::TokenSpan;

// The SK Math library!

// Needed for every library
pub fn register(env: &mut Environment) {
    env.define("sqrt".into(), Value::NativeFn(sqrt));

    env.define("sin".into(), Value::NativeFn(sin));
    env.define("cos".into(), Value::NativeFn(cos));
    env.define("tan".into(), Value::NativeFn(tan));

    env.define("log10".into(), Value::NativeFn(log10));
    env.define("log2".into(), Value::NativeFn(log2));
    env.define("ln".into(), Value::NativeFn(ln));

    env.define("exp".into(), Value::NativeFn(exp));
    env.define("abs".into(), Value::NativeFn(abs));
    env.define("min".into(), Value::NativeFn(min));
    env.define("max".into(), Value::NativeFn(max));

    env.define("deg".into(), Value::NativeFn(deg));
    env.define("rad".into(), Value::NativeFn(rad));
    env.define("atan2".into(), Value::NativeFn(atan2));

    env.define("PI".into(), Value::Number(std::f64::consts::PI));
    env.define("E".into(), Value::Number(std::f64::consts::E));

    // Interval Operators moved here
    env.define("width".into(), Value::NativeFn(width));
    env.define("mid".into(), Value::NativeFn(mid));
    env.define("intersection".into(), Value::NativeFn(intersection));
    env.define("union".into(), Value::NativeFn(union));
}

fn err(token: TokenSpan, msg: String) -> Error {
    Error {
        token,
        message: msg
    }
}

pub fn sqrt(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.sqrt())),
        Some(Value::Interval(min, max)) => {
            if *min < 0.0 {
                return Err(err(span, "Cannot take sqrt of negative interval".to_string()));
            }
            Ok(Value::Interval(min.sqrt(), max.sqrt()))
        }
        _ => Err(err(span, "sqrt() expects 1 number or interval".to_string())),
    }
}

pub fn sin(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.sin())),
        _ => Err(err(span, "sin() expects 1 number".to_string())),
    }
}

pub fn cos(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.cos())),
        _ => Err(err(span, "cos() expects 1 number".to_string())),
    }
}

pub fn tan(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.tan())),
        _ => Err(err(span, "tan() expects 1 number".to_string())),
    }
}

pub fn log10(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.log10())),
        _ => Err(err(span, "log10() expects 1 number".to_string())),
    }
}

pub fn log2(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.log2())),
        _ => Err(err(span, "log2() expects 1 number".to_string())),
    }
}

pub fn ln(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.ln())),
        _ => Err(err(span, "ln() expects 1 number".to_string())),
    }
}

pub fn exp(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.exp())),
        _ => Err(err(span, "exp() expects 1 number".to_string())),
    }
}

pub fn abs(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.abs())),
        _ => Err(err(span, "abs() expects 1 number".to_string())),
    }
}

pub fn min(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(err(span, "min() expects at least 2 numbers".to_string()));
    }

    let mut min_val = std::f64::INFINITY;
    for arg in args {
        match arg {
            Value::Number(n) => {
                if n < min_val {
                    min_val = n;
                }
            }
            _ => return Err(err(span, "min() expects only numbers".to_string())),
        }
    }
    Ok(Value::Number(min_val))
}

pub fn max(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(err(span, "max() expects at least 2 numbers".to_string()));
    }

    let mut max_val = std::f64::NEG_INFINITY;
    for arg in args {
        match arg {
            Value::Number(n) => {
                if n > max_val {
                    max_val = n;
                }
            }
            _ => return Err(err(span, "max() expects only numbers".to_string())),
        }
    }
    Ok(Value::Number(max_val))
}

pub fn deg(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.to_degrees())),
        _ => Err(err(span, "deg() expects 1 number".to_string())),
    }
}

pub fn rad(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.to_radians())),
        _ => Err(err(span, "rad() expects 1 number".to_string())),
    }
}

pub fn atan2(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(err(span, "atan2() expects exactly 2 numbers".to_string()));
    }

    match (&args[0], &args[1]) {
        (Value::Number(y), Value::Number(x)) => Ok(Value::Number(y.atan2(*x))),
        _ => Err(err(span, "atan2() expects only numbers".to_string())),
    }
}

// Interval Ops
pub fn width(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Interval(min, max)) => Ok(Value::Number(max - min)),
        _ => Err(err(span, "width() expects 1 interval".to_string())),
    }
}

pub fn mid(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Interval(min, max)) => Ok(Value::Number((min + max) / 2.0)),
        _ => Err(err(span, "mid() expects 1 interval".to_string())),
    }
}

pub fn intersection(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(err(span, "intersection() expects exactly 2 intervals".to_string()));
    }

    match (&args[0], &args[1]) {
        (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
            let new_min = min1.max(*min2);
            let new_max = max1.min(*max2);
            if new_min > new_max {
                Ok(Value::Interval(0.0, 0.0)) // No intersection
            } else {
                Ok(Value::Interval(new_min, new_max))
            }
        }
        _ => Err(err(span, "intersection() expects only intervals".to_string())),
    }
}

pub fn union(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(err(span, "union() expects exactly 2 intervals".to_string()));
    }

    match (&args[0], &args[1]) {
        (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
            let new_min = min1.min(*min2);
            let new_max = max1.max(*max2);
            Ok(Value::Interval(new_min, new_max))
        }
        _ => Err(err(span, "union() expects only intervals".to_string())),
    }
}