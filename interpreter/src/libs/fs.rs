use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::parser::lexer::TokenSpan;
use crate::core::error::Error;
use crate::core::value::{SKBool, Value};

use crate::Path;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

thread_local! {
    static OPEN_FILES: Mutex<HashMap<u64, File>> = Mutex::new(HashMap::new());
}
static FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

// This whole library is disabled in safe mode!
pub fn register(env: &mut Environment) {
    env.define("read".into(), Value::NativeFn(read));
    env.define("write".into(), Value::NativeFn(write));
    env.define("open".into(), Value::NativeFn(open));
    env.define("close".into(), Value::NativeFn(close));
    env.define("exists".into(), Value::NativeFn(exists));
    env.define("rename".into(), Value::NativeFn(rename));
}

fn ensure_unsafe(eval: &mut Evaluator, span: TokenSpan, fn_name: &str) -> Result<(), Error> {
    if eval.is_safe_mode() {
        return Err(eval.error(span, format!("{fn_name}() is disabled in --safe mode. Download the SK interpreter!")));
    }
    Ok(())
}

pub fn read(_args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    ensure_unsafe(eval, span.clone(), "read")?;

    if _args.len() != 1 {
        return Err(eval.error(span, "read() expects 1 argument"));
    }

    let file_id = match &_args[0] {
        Value::Number(id) => *id as u64,
        _ => return Err(eval.error(span, "read() expects a file handle (use open())")),
    };

    OPEN_FILES.with(|files| {
        let mut files = files.lock().unwrap();
        let file = files
            .get_mut(&file_id)
            .ok_or_else(|| eval.error(span.clone(), "Invalid file handle"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| eval.error(span.clone(), e.to_string()))?;
        Ok(Value::String(contents))
    })
}

// Supports Appending too!
pub fn write(_args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    ensure_unsafe(eval, span.clone(), "write")?;

    if _args.len() < 2 || _args.len() > 3 {
        return Err(eval.error(span, "write() expects 2 or 3 arguments"));
    }

    let file_id = match &_args[0] {
        Value::Number(id) => *id as u64,
        _ => return Err(eval.error(span, "write() expects a file handle (use open())")),
    };

    let append = if _args.len() == 3 {
        match &_args[2] {
            Value::Bool(SKBool::True) => true,
            Value::Bool(SKBool::False) => false,
            _ => return Err(eval.error(span, "write() append flag must be a bool")),
        }
    } else {
        false
    };

    OPEN_FILES.with(|files| {
        let mut files = files.lock().unwrap();
        let file = files
            .get_mut(&file_id)
            .ok_or_else(|| eval.error(span.clone(), "Invalid file handle"))?;
        if append {
            file.seek(SeekFrom::End(0))
                .map_err(|e| eval.error(span.clone(), e.to_string()))?;
        }
        let contents = _args[1].to_string();
        file.write_all(contents.as_bytes())
            .map_err(|e| eval.error(span.clone(), e.to_string()))?;
        Ok(Value::None)
    })
}

pub fn open(_args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    ensure_unsafe(eval, span.clone(), "open")?;

    if _args.is_empty() || _args.len() > 2 {
        return Err(eval.error(span, "open() expects 1 or 2 arguments"));
    }

    let path = match &_args[0] {
        Value::String(p) => p,
        _ => return Err(eval.error(span, "open() expects a string path")),
    };

    let mode = if _args.len() == 2 {
        match &_args[1] {
            Value::String(m) => m.as_str(),
            _ => return Err(eval.error(span, "open() mode must be a string")),
        }
    } else {
        "r"
    };

    let mut options = OpenOptions::new();
    match mode {
        "r" => {
            options.read(true);
        }
        "w" => {
            options.write(true).truncate(true).create(true);
        }
        "rw" | "r+" => {
            options.read(true).write(true).create(true);
        }
        _ => return Err(eval.error(span, "open() mode must be 'r', 'w', or 'rw'")),
    }

    let file = options
        .open(path)
        .map_err(|e| eval.error(span.clone(), format!("Failed to open '{}': {}", path, e)))?;

    let id = FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
    OPEN_FILES.with(|files| {
        files.lock().unwrap().insert(id, file);
    });

    Ok(Value::Number(id as f64))
}

pub fn close(_args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    ensure_unsafe(eval, span.clone(), "close")?;

    if _args.len() != 1 {
        return Err(eval.error(span, "close() expects 1 argument"));
    }

    let file_id = match &_args[0] {
        Value::Number(id) => *id as u64,
        _ => return Err(eval.error(span, "close() expects a file handle")),
    };

    OPEN_FILES.with(|files| {
        let mut files = files.lock().unwrap();
        if files.remove(&file_id).is_none() {
            return Err(eval.error(span, "Invalid file handle"));
        }
        Ok(Value::None)
    })
}

pub fn exists(_args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    ensure_unsafe(eval, span.clone(), "exists")?;

    if _args.len() != 1 {
        return Err(eval.error(span, "exists() expects 1 argument"));
    }

    let path = match &_args[0] {
        Value::String(p) => p,
        _ => return Err(eval.error(span, "exists() expects a string path")),
    };

    Ok(Value::Bool(if Path::new(path).exists() { SKBool::True } else { SKBool::False }))
}

pub fn rename(_args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    ensure_unsafe(eval, span.clone(), "rename")?;

    if _args.len() != 2 {
        return Err(eval.error(span, "rename() expects 2 arguments"));
    }

    let old_path = match &_args[0] {
        Value::String(p) => p,
        _ => return Err(eval.error(span, "rename() expects a string path")),
    };

    let new_path = match &_args[1] {
        Value::String(p) => p,
        _ => return Err(eval.error(span, "rename() expects a string path")),
    };

    std::fs::rename(old_path, new_path)
        .map_err(|e| eval.error(span, format!("Failed to rename '{}': {}", old_path, e)))?;

    Ok(Value::None)
}

// TODO: maybe delete, list etc...