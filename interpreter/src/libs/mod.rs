pub mod math;
pub mod os;
pub mod fs;
pub mod rand;
pub mod time;
pub mod units;
pub mod string;
pub mod http;

use crate::evaluator::env::Environment;
use std::collections::HashMap;

pub type LibRegisterFn = fn(&mut Environment);

pub fn get_library_registry() -> HashMap<String, LibRegisterFn> {
    let mut registry: HashMap<String, LibRegisterFn> = HashMap::new();
    
    registry.insert("math".to_string(), crate::libs::math::register);
    registry.insert("os".to_string(), crate::libs::os::register);
    registry.insert("fs".to_string(), crate::libs::fs::register);
    registry.insert("rand".to_string(), crate::libs::rand::register);
    registry.insert("time".to_string(), crate::libs::time::register);
    
    registry.insert("units".to_string(), crate::libs::units::register);

    registry.insert("string".to_string(), crate::libs::string::register);
    registry.insert("http".to_string(), crate::libs::http::register);
    
    registry
}