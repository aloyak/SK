use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::parser::lexer::TokenSpan;
use crate::core::error::Error;
use crate::core::value::Value;
use rand::Rng;
use rand::seq::SliceRandom;

pub fn register(env: &mut Environment) {
    env.define("random".into(), Value::NativeFn(random));
    env.define("range".into(), Value::NativeFn(random_range));
    env.define("rangeInt".into(), Value::NativeFn(random_range_int));
    env.define("shuffle".into(), Value::NativeFn(shuffle));
}

pub fn random(_args: Vec<Value>, _span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    let mut rng = rand::rng();
    let random_value: f64 = rng.random(); 
    Ok(Value::Number(random_value)) 
}

// Supports both 2 numbers or one interval
pub fn random_range(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.is_empty() || args.len() > 2 {
        return Err(eval.error(span, "range() expects 1 or 2 arguments"));
    }

    let (min, max) = if args.len() == 1 {
        match &args[0] {
            Value::Interval(min, max) => (*min, *max),
            _ => return Err(eval.error(span, "Single argument must be an interval")),
        }
    } else {
        let min = match &args[0] {
            Value::Number(n) => *n,
            _ => return Err(eval.error(span, "First argument must be a number")),
        };
        let max = match &args[1] {
            Value::Number(n) => *n,
            _ => return Err(eval.error(span, "Second argument must be a number")),
        };
        (min, max)
    };

    let mut rng = rand::rng();
    let random_value: f64 = rng.random_range(min..max);
    Ok(Value::Number(random_value))
}

pub fn random_range_int(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.is_empty() || args.len() > 2 {
        return Err(eval.error(span, "rangeInt() expects 1 or 2 arguments"));
    }

    let (min, max) = if args.len() == 1 {
        match &args[0] {
            Value::Interval(min, max) => (*min as i64, *max as i64),
            _ => return Err(eval.error(span, "Single argument must be an interval")),
        }
    } else {
        let min = match &args[0] {
            Value::Number(n) => *n as i64,
            _ => return Err(eval.error(span, "First argument must be a number")),
        };
        let max = match &args[1] {
            Value::Number(n) => *n as i64,
            _ => return Err(eval.error(span, "Second argument must be a number")),
        };
        (min, max)
    };

    let mut rng = rand::rng();
    let random_value: i64 = rng.random_range(min..max);
    Ok(Value::Number(random_value as f64))
}

pub fn shuffle(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(eval.error(span, "shuffle() expects exactly 1 argument"));
    }

    let list = match &args[0] {
        Value::Array(lst) => lst.clone(),
        _ => return Err(eval.error(span, "shuffle() expects an array")),
    };

    let mut rng = rand::rng();
    let mut shuffled_list = list.clone();
    shuffled_list.shuffle(&mut rng);
    
    Ok(Value::Array(shuffled_list))
}