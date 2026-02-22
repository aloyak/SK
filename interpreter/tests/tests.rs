use sk_lang::SKInterpreter;
use sk_lang::core::units::Unit;
use sk_lang::core::value::{SKBool, Value};

#[test]
fn evals_basic_expression() {
	let mut interpreter = SKInterpreter::new();
	let result1 = interpreter
		.execute_string("1 + 2".to_string())
		.expect("execution should succeed");

    let result2 = interpreter
		.execute_string("5 % 2".to_string())
		.expect("execution should succeed");

	assert_eq!(result1, Value::Number(3.0));
	assert_eq!(result2, Value::Number(1.0));
}

#[test]
fn evals_string_literal() {
	let mut interpreter = SKInterpreter::new();
	let result = interpreter
		.execute_string("'hello'".to_string())
		.expect("execution should succeed");

	assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn evals_function_call() {
    let mut interpreter = SKInterpreter::new();
    let result = interpreter
        .execute_string("possible(3 > 1)".to_string())
        .expect("execution should succeed");

    assert_eq!(result, Value::Bool(sk_lang::core::value::SKBool::True));
}

#[test]
fn evals_postfixes() {
    let mut interpreter = SKInterpreter::new();
    let result = interpreter
        .execute_string("let x = 5\nx++".to_string())
        .expect("execution should succeed");

    assert_eq!(result, Value::Number(6.0));
}

#[test]
fn evals_if_expression() {
    let mut interpreter = SKInterpreter::new();
    let result = interpreter
        .execute_string("if (partial) -> merge { 1 } else { 2 }".to_string())
        .expect("execution should succeed");

    assert_eq!(result, Value::Interval(1.0, 2.0));
}

#[test]
fn evals_logic() {
    let mut interpreter = SKInterpreter::new();
    let result1: Value = interpreter
        .execute_string("true && false || true".to_string())
        .expect("execution should succeed");

    let result2: Value = interpreter
        .execute_string("!false && (true || false)".to_string())
        .expect("execution should succeed");

    let result3: Value = interpreter
        .execute_string("false || false && true".to_string())
        .expect("execution should succeed");

    let result4: Value = interpreter
        .execute_string("partial && true".to_string())
        .expect("execution should succeed");

    assert_eq!(result1, Value::Bool(sk_lang::core::value::SKBool::True));
    assert_eq!(result2, Value::Bool(sk_lang::core::value::SKBool::True));
    assert_eq!(result3, Value::Bool(sk_lang::core::value::SKBool::False));
    assert_eq!(result4, Value::Bool(sk_lang::core::value::SKBool::Partial));
}

#[test]
fn evals_scopes() {
    let mut interpreter = SKInterpreter::new();
    let result = interpreter
        .execute_string("let x = 10\n{ let x = 20\nx }".to_string())
        .expect("execution should succeed");

    assert_eq!(result, Value::Number(20.0));
}

#[test]
fn evals_loops() {
    let mut interpreter = SKInterpreter::new();
    let result = interpreter
        .execute_string("let n = 0\nloop { n++\n if n > 10 { break } }\nn".to_string())
        .expect("execution should succeed");

    assert_eq!(result, Value::Number(11.0));
}

#[test]
fn evals_symbolics() {
    let mut interpreter = SKInterpreter::new();
    let result1 = interpreter
        .execute_string("let x = 0\nsymbolic z = x+1\nlet x = 1\nresolve(z)".to_string())
        .expect("execution should succeed");

    let result2 = interpreter
        .execute_string("let y = 2\nquiet k = y*3\ny = 4\nk".to_string())
        .expect("execution should succeed");

    let result3 = interpreter
        .execute_string("let a = 2\nsymbolic b = a * 3\nb".to_string())
        .expect("execution should succeed");

    assert_eq!(result1, Value::Number(2.0));
    assert_eq!(result2, Value::Number(12.0));
    assert!(matches!(result3, Value::Symbolic { is_quiet: false, .. }));
}

#[test]
fn evals_increments() {
    let mut interpreter = SKInterpreter::new();
    let result1 = interpreter.execute_string("let x = 2\nx++".to_string())
        .expect("execution should succeed");
    let result2 = interpreter.execute_string("let x = 2\nx--".to_string())
        .expect("execution should succeed");
    
    assert_eq!(result1, Value::Number(3.0));
    assert_eq!(result2, Value::Number(1.0));
}

#[test]
fn evals_intervals() {
    let mut interpreter = SKInterpreter::new();
    let result = interpreter
        .execute_string("let x = [-10..10]\nx".to_string())
        .expect("execution should succeed");

    assert_eq!(result, Value::Interval(-10.0, 10.0));
}

#[test]
fn evals_arrays() {
    let mut interpreter = SKInterpreter::new();
    let result1 = interpreter
        .execute_string("let arr = [1, 2, 3]\narr[1]".to_string())
        .expect("execution should succeed");

    let result2 = interpreter
        .execute_string("let arr = [10, 20, 30]\narr.len()".to_string())
        .expect("execution should succeed");

    let result3 = interpreter
        .execute_string("let arr = [5, 10, 15]\narr.push(20)\narr[3]".to_string())
        .expect("execution should succeed");    

    let result4 = interpreter
        .execute_string("let arr = []\narr.len()".to_string())
        .expect("execution should succeed");

    assert_eq!(result1, Value::Number(2.0));
    assert_eq!(result2, Value::Number(3.0));
    assert_eq!(result3, Value::Number(20.0));
    assert_eq!(result4, Value::Number(0.0));
}

#[test]
fn evals_units() {
    let mut interpreter = SKInterpreter::new();
    let result1 = interpreter
        .execute_string("import units\nlet speed = 60 m/s\nspeed".to_string())
        .expect("execution should succeed");

    let result2 = interpreter
        .execute_string("import units\nlet time = 2 s\ntime".to_string())
        .expect("execution should succeed");

    let result3 = interpreter
        .execute_string("import units\nlet distance = [0..1] km\ndistance".to_string())
        .expect("execution should succeed");

    let ms_unit = Unit::base("m").div(&Unit::base("s"));
    let s_unit = Unit::base("s");
    let km_unit = Unit::base("m");

    assert_eq!(result1, Value::Quantity { value: Box::new(Value::Number(60.0)), unit: ms_unit });
    assert_eq!(result2, Value::Quantity { value: Box::new(Value::Number(2.0)), unit: s_unit });
    assert_eq!(result3, Value::Quantity { value: Box::new(Value::Interval(0.0, 1000.0)), unit: km_unit });
}

#[test]
fn evals_match() {
    let mut interpreter = SKInterpreter::new();
    let result = interpreter
        .execute_string("match 2 { 1 => false\n2 => true\nany => partial }".to_string())
        .expect("execution should succeed");
    assert_eq!(result, Value::Bool(SKBool::True));
}

#[test]
fn evals_try_catch() {
    let mut interpreter = SKInterpreter::new();
    let result = interpreter
        .execute_string("unknown result\ntry { panic! } catch { result = 'caught' }\nresult".to_string())
        .expect("execution should succeed");
    assert_eq!(result, Value::String("caught".to_string()));
}

#[test]
fn library_import() {
    let mut interpreter = SKInterpreter::new();
    let result = interpreter
        .execute_string("import math as mt\nmt.sqrt(16)".to_string())
        .expect("execution should succeed");

    assert_eq!(result, Value::Number(4.0));
}

#[test]
fn library_import_alias() {
    let mut interpreter = SKInterpreter::new();
    let result = interpreter
        .execute_string("import math as m\nm.sqrt(25)".to_string())
        .expect("execution should succeed");

    assert_eq!(result, Value::Number(5.0));
}