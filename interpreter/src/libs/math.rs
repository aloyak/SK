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

    env.define("truncate".into(), Value::NativeFn(truncate));
    env.define("floor".into(), Value::NativeFn(floor));
    env.define("round".into(), Value::NativeFn(round));

    // Interval Operators moved here
    env.define("width".into(), Value::NativeFn(width));
    env.define("mid".into(), Value::NativeFn(mid));
    env.define("intersection".into(), Value::NativeFn(intersection));
    env.define("union".into(), Value::NativeFn(union));
}

pub fn sqrt(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.sqrt())),
        Some(Value::Interval(min, max)) => {
            if *min < 0.0 {
                return Err(eval.error(span, "Cannot take sqrt of negative interval"));
            }
            Ok(Value::Interval(min.sqrt(), max.sqrt()))
        }
        _ => Err(eval.error(span, "sqrt() expects 1 number or interval")),
    }
}

pub fn sin(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.sin())),
        _ => Err(eval.error(span, "sin() expects 1 number")),
    }
}

pub fn cos(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.cos())),
        _ => Err(eval.error(span, "cos() expects 1 number")),
    }
}

pub fn tan(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.tan())),
        _ => Err(eval.error(span, "tan() expects 1 number")),
    }
}

pub fn log10(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.log10())),
        _ => Err(eval.error(span, "log10() expects 1 number")),
    }
}

pub fn log2(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.log2())),
        _ => Err(eval.error(span, "log2() expects 1 number")),
    }
}

pub fn ln(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.ln())),
        _ => Err(eval.error(span, "ln() expects 1 number")),
    }
}

pub fn exp(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.exp())),
        _ => Err(eval.error(span, "exp() expects 1 number")),
    }
}

pub fn abs(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.abs())),
        _ => Err(eval.error(span, "abs() expects 1 number")),
    }
}

pub fn min(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(eval.error(span, "min() expects at least 2 numbers"));
    }

    let mut min_val = std::f64::INFINITY;
    for arg in args {
        match arg {
            Value::Number(n) => {
                if n < min_val {
                    min_val = n;
                }
            }
            _ => return Err(eval.error(span, "min() expects only numbers")),
        }
    }
    Ok(Value::Number(min_val))
}

pub fn max(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(eval.error(span, "max() expects at least 2 numbers"));
    }

    let mut max_val = std::f64::NEG_INFINITY;
    for arg in args {
        match arg {
            Value::Number(n) => {
                if n > max_val {
                    max_val = n;
                }
            }
            _ => return Err(eval.error(span, "max() expects only numbers")),
        }
    }
    Ok(Value::Number(max_val))
}

pub fn deg(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.to_degrees())),
        _ => Err(eval.error(span, "deg() expects 1 number")),
    }
}

pub fn rad(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.to_radians())),
        _ => Err(eval.error(span, "rad() expects 1 number")),
    }
}

pub fn atan2(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(eval.error(span, "atan2() expects exactly 2 numbers"));
    }

    match (&args[0], &args[1]) {
        (Value::Number(y), Value::Number(x)) => Ok(Value::Number(y.atan2(*x))),
        _ => Err(eval.error(span, "atan2() expects only numbers")),
    }
}

pub fn truncate(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(eval.error(span, "truncate() expects two numbers"));
    }

    match (&args[0], &args[1]) {
        (Value::Number(n), Value::Number(decimals)) => {
            let factor = 10f64.powf(*decimals);
            Ok(Value::Number((n * factor).trunc() / factor))
        }
        _ => Err(eval.error(span, "truncate() expects only numbers")),
    }
}

pub fn floor(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.floor())),
        _ =>
            Err(eval.error(span, "floor() expects 1 number")),
    }
}

pub fn round(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.round())),
        _ =>
            Err(eval.error(span, "round() expects 1 number")),
    }
}

// Interval Ops
pub fn width(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Interval(min, max)) => Ok(Value::Number(max - min)),
        _ => Err(eval.error(span, "width() expects 1 interval")),
    }
}

pub fn mid(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Interval(min, max)) => Ok(Value::Number((min + max) / 2.0)),
        _ => Err(eval.error(span, "mid() expects 1 interval")),
    }
}

pub fn intersection(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(eval.error(span, "intersection() expects exactly 2 intervals"));
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
        _ => Err(eval.error(span, "intersection() expects only intervals")),
    }
}

pub fn union(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(eval.error(span, "union() expects exactly 2 intervals"));
    }

    match (&args[0], &args[1]) {
        (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
            let new_min = min1.min(*min2);
            let new_max = max1.max(*max2);
            Ok(Value::Interval(new_min, new_max))
        }
        _ => Err(eval.error(span, "union() expects only intervals")),
    }
}