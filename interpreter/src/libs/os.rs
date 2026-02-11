use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::parser::lexer::TokenSpan;
use crate::core::error::Error;
use crate::core::value::Value;

use std::env::consts::OS;
use std::process::Command;

pub fn register(env: &mut Environment) {
    env.define("name".into(), Value::NativeFn(name));
    env.define("command".into(), Value::NativeFn(command));
    env.define("clear".into(), Value::NativeFn(clear));
}

pub fn name(_args: Vec<Value>, _span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    Ok(Value::String(OS.to_string()))
}

pub fn command(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    if eval.is_safe_mode() {
        return Err(eval.error(span, "os.command() is disabled in --safe mode. Download the SK interpreter!"));
    }

    if args.is_empty() {
        return Err(eval.error(span, "command() requires at least one argument"));
    }

    let cmd_str = match &args[0] {
        Value::String(s) => s,
        _ => return Err(eval.error(span, "command() expects a string argument")),
    };

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd_str])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .output()
    };

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            
            if output.status.success() {
                Ok(Value::String(stdout))
            } else {
                Err(eval.error(span, format!("Command failed: {}", stderr)))
            }
        }
        Err(e) => Err(eval.error(span, format!("Failed to execute command: {}", e))),
    }
}

pub fn clear(_args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status()
    } else {
        Command::new("clear").status()
    };

    match status {
        Ok(status) => {
            if status.success() {
                Ok(Value::None)
            } else {
                Err(eval.error(span, "Failed to clear the console"))
            }
        }
        Err(e) => Err(eval.error(span, e.to_string())),
    }
}