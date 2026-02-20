use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::parser::lexer::TokenSpan;
use crate::core::error::Error;
use crate::core::value::Value;
use std::time::Duration;

fn ensure_unsafe(eval: &mut Evaluator, span: TokenSpan, fn_name: &str) -> Result<(), Error> {
    if eval.is_safe_mode() {
        return Err(eval.error(span, format!("{fn_name}() is disabled in --safe mode. Download the SK interpreter!")));
    }
    Ok(())
}

pub fn register(env: &mut Environment) {
    env.define("get".into(), Value::NativeFn(get));    
    env.define("post".into(), Value::NativeFn(post));
}

pub fn get(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    ensure_unsafe(eval, span.clone(), "http.get")?;

    if args.len() != 1 {
        return Err(eval.error(span, "get() expects 1 argument"));
    }

    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(eval.error(span.clone(), "get() expects a string URL")),
    };

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| eval.error(span.clone(), format!("Failed to create HTTP client: {}", e)))?;

    let response = client
        .get(&url)
        .send()
        .map_err(|e| eval.error(span.clone(), format!("HTTP request failed: {}", e)))?;

    if !response.status().is_success() {
        return Err(eval.error(
            span.clone(),
            format!("HTTP request failed with status: {}", response.status()),
        ));
    }

    let body = response
        .text()
        .map_err(|e| eval.error(span, format!("Failed to read response body: {}", e)))?;

    Ok(Value::String(body))
}

pub fn post(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    ensure_unsafe(eval, span.clone(), "http.post")?;

    if args.len() != 2 {
        return Err(eval.error(span, "post() expects 2 arguments"));
    }

    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(eval.error(span.clone(), "post() expects a string URL as first argument")),
    };

    let body = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(eval.error(span.clone(), "post() expects a string body as second argument")),
    };

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| eval.error(span.clone(), format!("Failed to create HTTP client: {}", e)))?;

    let response = client
        .post(&url)
        .header("Content-Type", "text/plain")
        .body(body)
        .send()
        .map_err(|e| eval.error(span.clone(), format!("HTTP request failed: {}", e)))?;

    if !response.status().is_success() {
        return Err(eval.error(
            span.clone(),
            format!("HTTP request failed with status: {}", response.status()),
        ));
    }

    let response_body = response
        .text()
        .map_err(|e| eval.error(span, format!("Failed to read response body: {}", e)))?;

    Ok(Value::String(response_body))
}
