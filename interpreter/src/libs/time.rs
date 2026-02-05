use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::parser::lexer::TokenSpan;
use crate::core::error::Error;
use crate::core::value::Value; 

use std::time::UNIX_EPOCH;
use std::sync::Mutex;
use std::collections::HashMap;

thread_local! {
    static TIMERS: Mutex<HashMap<u64, std::time::Instant>> = Mutex::new(HashMap::new());
}
static TIMER_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn register(env: &mut Environment) {
    env.define("now".into(), Value::NativeFn(now));
    env.define("format".into(), Value::NativeFn(format));
    env.define("sleep".into(), Value::NativeFn(sleep));

    env.define("startTimer".into(), Value::NativeFn(start_timer));
    env.define("stopTimer".into(), Value::NativeFn(stop_timer));
}

fn err(token: TokenSpan, msg: String) -> Error {
    Error {
        token,
        message: msg
    }
}

pub fn now(_args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    use std::time::{SystemTime, UNIX_EPOCH};
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => Ok(Value::Number(duration.as_secs_f64())),
        Err(e) => Err(err(span, e.to_string())),
    }
}

pub fn format(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    use chrono::{DateTime, Utc};
    // YYYY-MM-DD: HH:MM:SS
    match args.first() {
        Some(Value::Number(n)) => {
            let dt = DateTime::<Utc>::from(UNIX_EPOCH + std::time::Duration::from_secs_f64(*n));
            Ok(Value::String(dt.format("%Y-%m-%d: %H:%M:%S").to_string()))
        }
        _ => Err(err(span, "format() expects 1 number".to_string())),
    }
}

// In Seconds
pub fn sleep(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    use std::thread;
    use std::time::Duration;
    match args.first() {
        Some(Value::Number(n)) => {
            if *n < 0.0 {
                return Err(err(span, "Cannot sleep for negative time!".to_string()));
            }
            thread::sleep(Duration::from_secs_f64(*n));
            Ok(Value::None)
        }
        Some(Value::Interval(min, max)) => {
            let sleep_time = (min + max) / 2.0;
            thread::sleep(Duration::from_secs_f64(sleep_time));
            Ok(Value::None)
        }
        _ => Err(err(span, "sleep() expects 1 value or interval".to_string())),
    }
}

// Timer
// start_timer() -> creates a new thread with a timer, returns timer id
// stop_timer() -> stops the timer and returns the elapsed time in seconds (error if no timer)

pub fn start_timer(_args: Vec<Value>, _span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    let id = TIMER_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    TIMERS.with(|timers| {
        timers.lock().unwrap().insert(id, std::time::Instant::now());
    });
    Ok(Value::Number(id as f64))
}

pub fn stop_timer(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(timer_id)) => {
            let timer_id = *timer_id as u64;
            TIMERS.with(|timers| {
                let mut timers = timers.lock().unwrap();
                match timers.remove(&timer_id) {
                    Some(start) => {
                        let elapsed = start.elapsed().as_secs_f64();
                        Ok(Value::Number(elapsed))
                    }
                    None => Err(err(span, "Timer not found".to_string())),
                }
            })
        }
        _ => Err(err(span, "stopTimer() expects 1 number (timer ID)".to_string())),
    }
}