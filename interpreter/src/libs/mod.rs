pub mod math;
pub mod os;
pub mod fs;
pub mod rand;
pub mod time;
pub mod units;

use crate::evaluator::env::Environment;
use std::collections::HashMap;

pub type LibRegisterFn = fn(&mut Environment);

pub fn get_library_registry() -> HashMap<String, LibRegisterFn> {
    let mut registry: HashMap<String, LibRegisterFn> = HashMap::new();
    
    // Standard libraries: 
    registry.insert("math".to_string(), crate::libs::math::register);

    // Units Lib as well
    registry.insert("units".to_string(), crate::libs::units::register);

    registry.insert("os".to_string(), crate::libs::os::register);
    registry.insert("fs".to_string(), crate::libs::fs::register);
    registry.insert("rand".to_string(), crate::libs::rand::register);
    registry.insert("time".to_string(), crate::libs::time::register);
    
    registry
}