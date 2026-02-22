use crate::core::units::Unit;
use crate::core::value::Value;
use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::parser::lexer::TokenSpan;
use crate::core::error::Error;

fn unit_value(symbol: &str) -> Value {
	Value::Quantity {
		value: Box::new(Value::Number(1.0)),
		unit: Unit::base(symbol),
	}
}

fn unit_from(unit: Unit) -> Value {
	Value::Quantity {
		value: Box::new(Value::Number(1.0)),
		unit,
	}
}

fn scaled_unit_value(symbol: &str, scale: f64) -> Value {
	Value::Quantity {
		value: Box::new(Value::Number(scale)),
		unit: Unit::base(symbol),
	}
}

pub fn register(env: &mut Environment) {
	// SI base units
	env.define("m".into(), unit_value("m"));
	env.define("s".into(), unit_value("s"));
	env.define("kg".into(), unit_value("kg"));
	env.define("L".into(), unit_value("L"));
	env.define("A".into(), unit_value("A"));
	env.define("K".into(), unit_value("K"));
	env.define("mol".into(), unit_value("mol"));
	env.define("cd".into(), unit_value("cd"));

	// Derived
	let hz = Unit::dimensionless().div(&Unit::base("s"));
	let newton = Unit::base("kg")
		.mul(&Unit::base("m"))
		.div(&Unit::base("s").pow(2));
	let joule = newton.mul(&Unit::base("m"));
	let watt = joule.div(&Unit::base("s"));

	let pascal = newton.div(&Unit::base("m").pow(2));
    
	let coulomb = Unit::base("A").mul(&Unit::base("s"));
	let volt = watt.div(&Unit::base("A"));
	let ohm = volt.div(&Unit::base("A"));

	env.define("Hz".into(), unit_from(hz));
	env.define("N".into(), unit_from(newton));
	env.define("J".into(), unit_from(joule));
	env.define("W".into(), unit_from(watt));
	env.define("Pa".into(), unit_from(pascal));
	env.define("C".into(), unit_from(coulomb));
	env.define("V".into(), unit_from(volt));
	env.define("Ohm".into(), unit_from(ohm));

    // Common prefixes
	env.define("km".into(), scaled_unit_value("m", 1000.0));
	env.define("cm".into(), scaled_unit_value("m", 0.01));
	env.define("mm".into(), scaled_unit_value("m", 0.001));

	env.define("min".into(), scaled_unit_value("s", 60.0));
	env.define("h".into(), scaled_unit_value("s", 3600.0));

	env.define("g".into(), scaled_unit_value("kg", 0.001));
	env.define("mg".into(), scaled_unit_value("kg", 0.000001));

	// Temperature scales
	env.define("dC".into(), Value::Quantity {
		value: Box::new(Value::Number(1.0)),
		unit: Unit::base("dC"),
	});
	env.define("dK".into(), Value::Quantity {
		value: Box::new(Value::Number(1.0)),
		unit: Unit::base("dK"),
	});

	env.define("define".into(), Value::NativeFn(define));
}

fn define(args: Vec<Value>, span: TokenSpan, eval: &mut Evaluator) -> Result<Value, Error> {
	if args.len() != 2 {
		return Err(eval.error(span, "define() expects name and value"));
	}

	let name = match &args[0] {
		Value::String(s) => s.clone(),
		_ => return Err(eval.error(span, "define() name must be a string")),
	};

	let quantity = match &args[1] {
		Value::Quantity { .. } => args[1].clone(),
		_ => return Err(eval.error(span, "define() value must be a unit quantity")),
	};

	eval.env.borrow_mut().define(name.clone(), quantity.clone());
	if let Ok(Value::Module(units_mod)) = eval.env.borrow().get("units") {
		units_mod.borrow_mut().define(name, quantity);
	}

	Ok(Value::None)
}
