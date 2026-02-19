use crate::core::value::SKBool;

pub fn and(a: SKBool, b: SKBool) -> SKBool {
    match (a, b) {
        (SKBool::False, _) | (_, SKBool::False) => SKBool::False,
        (SKBool::True, SKBool::True) => SKBool::True,
        _ => SKBool::Partial,
    }
}

pub fn or(a: SKBool, b: SKBool) -> SKBool {
    match (a, b) {
        (SKBool::True, _) | (_, SKBool::True) => SKBool::True,
        (SKBool::False, SKBool::False) => SKBool::False,
        _ => SKBool::Partial,
    }
}

pub fn not(a: SKBool) -> SKBool {
    match a {
        SKBool::True => SKBool::False,
        SKBool::False => SKBool::True,
        SKBool::Partial => SKBool::Partial,
    }
}

pub fn compare_nums(a: f64, b: f64, op: &str) -> SKBool {
    let res = match op {
        "==" => a == b,
        "!=" => a != b,
        ">" => a > b,
        "<" => a < b,
        ">=" => a >= b,
        "<=" => a <= b,
        _ => return SKBool::Partial,
    };
    if res { SKBool::True } else { SKBool::False }
}

pub fn compare_intervals(min1: f64, max1: f64, min2: f64, max2: f64, op: &str) -> SKBool {
    match op {
        ">" => {
            if min1 > max2 { SKBool::True }
            else if max1 <= min2 { SKBool::False }
            else { SKBool::Partial }
        }
        "<" => {
            if max1 < min2 { SKBool::True }
            else if min1 >= max2 { SKBool::False }
            else { SKBool::Partial }
        }
        ">=" => {
            if min1 >= max2 { SKBool::True }
            else if max1 < min2 { SKBool::False }
            else { SKBool::Partial }
        }
        "<=" => {
            if max1 <= min2 { SKBool::True }
            else if min1 > max2 { SKBool::False }
            else { SKBool::Partial }
        }
        "==" => {
            if min1 == min2 && max1 == max2 { SKBool::True }
            else if max1 < min2 || min1 > max2 { SKBool::False }
            else { SKBool::Partial }
        }
        "!=" => {
            if min1 == min2 && max1 == max2 { SKBool::False }
            else if max1 < min2 || min1 > max2 { SKBool::True }
            else { SKBool::Partial }
        }
        _ => SKBool::Partial,
    }
}