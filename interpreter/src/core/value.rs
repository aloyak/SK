use core::fmt;
use crate::parser::ast::Expr;
use crate::parser::lexer::Token;
use crate::core::logic;

#[derive(Debug, Clone, PartialEq)]
pub enum SKBool {
    True,
    False,
    Partial
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(SKBool),
    Interval(f64, f64),
    Unknown,
    Symbolic {
        expression: Box<Expr>,
        is_quiet: bool,
    },
    None,
}

impl Value {
    pub fn is_symbolic_or_unknown(&self) -> bool {
        matches!(self, Value::Symbolic { .. } | Value::Unknown)
    }

    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(s1), Value::String(s2)) => Ok(Value::String(format!("{}{}", s1, s2))),
            
            (Value::Interval(min, max), Value::Number(n)) |
            (Value::Number(n), Value::Interval(min, max)) => Ok(Value::Interval(min + n, max + n)),
            
            (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
                Ok(Value::Interval(min1 + min2, max1 + max2))
            },
            
            _ => Err("Invalid types for addition".to_string()),
        }
    }

    pub fn sub(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (l, r) if l == r && !matches!(l, Value::Unknown | Value::Symbolic { .. }) => Ok(Value::Number(0.0)),
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),

            (Value::Interval(min, max), Value::Number(n)) => Ok(Value::Interval(min - n, max - n)),
            (Value::Number(n), Value::Interval(min, max)) => Ok(Value::Interval(n - max, n - min)),

            (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
                Ok(Value::Interval(min1 - max2, max1 - min2))
            },

            _ => Err("Invalid types for subtraction".to_string()),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(n), _) | (_, Value::Number(n)) if *n == 0.0 => Ok(Value::Number(0.0)),

            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),

            (Value::Interval(min, max), Value::Number(n)) |
            (Value::Number(n), Value::Interval(min, max)) => {
                let a = min * n;
                let b = max * n;
                Ok(Value::Interval(a.min(b), a.max(b)))
            },

            (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
                let p = [min1 * min2, min1 * max2, max1 * min2, max1 * max2];
                Ok(Value::Interval(
                    p.iter().copied().fold(f64::INFINITY, f64::min),
                    p.iter().copied().fold(f64::NEG_INFINITY, f64::max)
                ))
            },

            _ => Err("Invalid types for multiplication".to_string()),
        }
    }

    pub fn div(&self, other: &Value) -> Result<Value, String> {
        if self == other {
            match self {
                Value::Number(n) if *n != 0.0 => return Ok(Value::Number(1.0)),
                Value::Interval(min, max) if *min > 0.0 || *max < 0.0 => return Ok(Value::Number(1.0)),
                _ => {} 
            }
        }

        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 { return Err("Division by zero".to_string()); }
                Ok(Value::Number(a / b))
            },
            _ => Err("Division only supported for distinct Numbers currently".to_string()),
        }
    }

    pub fn pow(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(*b))),
            
            (Value::Interval(min, max), Value::Number(n)) => {
                if n % 2.0 == 0.0 {
                    let p1 = min.powf(*n);
                    let p2 = max.powf(*n);
                    let mut low = p1.min(p2);
                    let high = p1.max(p2);
                    if *min <= 0.0 && *max >= 0.0 { 
                        low = 0.0; 
                    }
                    Ok(Value::Interval(low, high))
                } else {
                    let p1 = min.powf(*n);
                    let p2 = max.powf(*n);
                    Ok(Value::Interval(p1.min(p2), p1.max(p2)))
                }
            },
            _ => Err("Invalid types for exponentiation".to_string()),
        }
    }

    pub fn compare(&self, other: &Value, op: &Token) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                 let op_str = match op {
                    Token::EqualEqual => "==",
                    Token::BangEqual => "!=",
                    Token::Greater => ">",
                    Token::GreaterEqual => ">=",
                    Token::Less => "<",
                    Token::LessEqual => "<=",
                    _ => return Ok(Value::Bool(SKBool::Partial)),
                };
                Ok(Value::Bool(logic::compare_nums(*a, *b, op_str)))
            },
            (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
                let op_str = match op {
                    Token::Greater => ">",
                    Token::Less => "<",
                    Token::GreaterEqual => ">=",
                    Token::LessEqual => "<=",
                    Token::EqualEqual => "==",
                    Token::BangEqual => "!=",
                    _ => return Ok(Value::Bool(SKBool::Partial)),
                };
                Ok(Value::Bool(logic::compare_intervals(*min1, *max1, *min2, *max2, op_str)))
            },
            (Value::String(s1), Value::String(s2)) => match op {
                Token::EqualEqual => Ok(Value::Bool(if s1 == s2 { SKBool::True } else { SKBool::False })),
                Token::BangEqual => Ok(Value::Bool(if s1 != s2 { SKBool::True } else { SKBool::False })),
                _ => Err("Invalid comparison for strings".to_string()),
            },
            
            (Value::Interval(min, max), Value::Number(n)) => 
                Value::Interval(*min, *max).compare(&Value::Interval(*n, *n), op),
            
            (Value::Number(n), Value::Interval(min, max)) => 
                Value::Interval(*n, *n).compare(&Value::Interval(*min, *max), op),

            _ => Ok(Value::Bool(SKBool::Partial)),
        }
    }

    pub fn logic(&self, other: &Value, op: &Token) -> Result<Value, String> {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => match op {
                Token::And => Ok(Value::Bool(logic::and(a.clone(), b.clone()))),
                Token::Or => Ok(Value::Bool(logic::or(a.clone(), b.clone()))),
                _ => Err("Invalid logic operator".to_string()),
            },
            _ => Err("Logic operations require booleans".to_string()),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(SKBool::True) => write!(f, "true"),
            Value::Bool(SKBool::False) => write!(f, "false"),
            Value::Bool(SKBool::Partial) => write!(f, "partial"),
            Value::Interval(min, max) => write!(f, "[{}..{}]", min, max),
            Value::Symbolic { .. } => write!(f, "<symbolic>"),
            Value::Unknown => write!(f, "unknown"),
            Value::None => write!(f, "none"),
        }
    }
}