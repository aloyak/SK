use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::core::value::Value;
use crate::evaluator::builtins;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, Value>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Self {
            values: HashMap::new(),
            enclosing: None,
        };

        let defs: [(&str, crate::core::value::NativeFn); 15] = [
            ("print", builtins::print),
            ("write", builtins::write),
            ("input", builtins::input),
            ("num", builtins::num),
            ("str", builtins::str),
            ("resolve", builtins::resolve),
            ("certain", builtins::certain),
            ("impossible", builtins::impossible),
            ("possible", builtins::possible),
            ("known", builtins::known),
            ("kind", builtins::kind),
            ("intersect", builtins::intersect),
            ("union", builtins::union),
            ("mid", builtins::mid),
            ("width", builtins::width),
        ];

        for (name, func) in defs {
            env.define(name.to_string(), Value::NativeFn(func));
        }

        env
    }

    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow().get(name);
        }

        Err(format!("Use of undefined variable '{}'", name))
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            return Ok(());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }

        Err(format!("Undefined variable '{}'", name))
    }
}