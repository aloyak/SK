pub mod math;
pub mod fs;
pub mod rand;
pub mod time;

use crate::evaluator::env::Environment;
use std::collections::HashMap;

pub type LibRegisterFn = fn(&mut Environment);

pub fn get_library_registry() -> HashMap<String, LibRegisterFn> {
    let mut registry: HashMap<String, LibRegisterFn> = HashMap::new();
    
    // Standard libraries: 
    registry.insert("math".to_string(), crate::libs::math::register);

    registry.insert("fs".to_string(), crate::libs::fs::register);
    registry.insert("rand".to_string(), crate::libs::rand::register);
    registry.insert("time".to_string(), crate::libs::time::register);
    
    registry
}