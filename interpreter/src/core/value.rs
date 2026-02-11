use core::fmt;
use crate::parser::ast::{Expr, Parameter, Stmt};
use crate::parser::lexer::{Token, TokenSpan};
use crate::core::logic;
use crate::core::error::Error;

use crate::evaluator::env::Environment;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
pub enum SKBool {
    True,
    False,
    Partial
}

pub type NativeFn = fn(Vec<Value>, TokenSpan, &mut crate::evaluator::eval::Evaluator) -> Result<Value, Error>;

#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<Parameter>,
    pub body: Vec<Stmt>,
    pub closure: Rc<RefCell<Environment>>, 
    pub is_public: bool,
}

#[derive(Debug, Clone)]
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
    NativeFn(NativeFn),
    Function(Function),
    Module(Rc<RefCell<Environment>>),
    None,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Interval(a_min, a_max), Value::Interval(b_min, b_max)) => a_min == b_min && a_max == b_max,
            (Value::Unknown, Value::Unknown) => true,
            (Value::Symbolic { expression: e1, is_quiet: q1 }, Value::Symbolic { expression: e2, is_quiet: q2 }) => e1 == e2 && q1 == q2,
            (Value::None, Value::None) => true,
            _ => false,
        }
    }
}

impl Value {
    pub fn is_symbolic_or_unknown(&self) -> bool {
        matches!(self, Value::Symbolic { .. } | Value::Unknown)
    }

    fn format_expr(expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => {
                let l = Self::format_expr(left);
                let r = Self::format_expr(right);
                let op = match operator.token {
                    Token::Plus => "+",
                    Token::Minus => "-",
                    Token::Star => "*",
                    Token::Slash => "/",
                    Token::Caret => "^",
                    Token::Modulo => "%",
                    Token::EqualEqual => "==",
                    Token::BangEqual => "!=",
                    Token::Greater => ">",
                    Token::GreaterEqual => ">=",
                    Token::Less => "<",
                    Token::LessEqual => "<=",
                    Token::And => "&&",
                    Token::Or => "||",
                    _ => "?",
                };
                format!("({} {} {})", l, op, r)
            }
            Expr::Literal { value } => match &value.token {
                Token::Number(n) => n.to_string(),
                Token::String(s) => s.clone(),
                Token::True => "true".to_string(),
                Token::False => "false".to_string(),
                Token::Partial => "partial".to_string(),
                Token::Unknown => "unknown".to_string(),
                _ => format!("{:?}", value.token),
            },
            Expr::Variable { name } => {
                if let Token::Identifier(n) = &name.token { n.clone() } else { format!("{:?}", name.token) }
            }
            Expr::Grouping { expression } => format!("({})", Self::format_expr(expression)),
            Expr::Postfix { name, operator } => {
                let n = match &name.token {
                    Token::Identifier(s) => s.as_str(),
                    _ => "?",
                };
                let op = match operator.token {
                    Token::Increment => "++",
                    Token::Decrement => "--",
                    _ => "?",
                };
                format!("{}{}", n, op)
            }
            _ => "...".to_string(),
        }
    }

    fn err(msg: String) -> Error {
        Error::new(
            TokenSpan {
                token: Token::Unknown,
                line: 0,
                column: 0,
            },
            msg,
        )
    }

    pub fn add(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(s1), Value::String(s2)) => Ok(Value::String(format!("{}{}", s1, s2))),

            (Value::Interval(min, max), Value::Number(n)) | (Value::Number(n), Value::Interval(min, max)) => {
                Ok(Value::Interval(min + n, max + n))
            }
            (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
                Ok(Value::Interval(min1 + min2, max1 + max2))
            },

            _ => Err(Self::err("Invalid types for addition".to_string())),
        }
    }

    pub fn sub(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (l, r) if l == r && !l.is_symbolic_or_unknown() => Ok(Value::Number(0.0)),
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),

            (Value::Interval(min, max), Value::Number(n)) => Ok(Value::Interval(min - n, max - n)),
            (Value::Number(n), Value::Interval(min, max)) => Ok(Value::Interval(n - max, n - min)),

            (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
                Ok(Value::Interval(min1 - max2, max1 - min2))
            },

            _ => Err(Self::err("Invalid types for subtraction".to_string())),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(n), _) | (_, Value::Number(n)) if *n == 0.0 => Ok(Value::Number(0.0)),
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),

            (Value::Interval(min, max), Value::Number(n)) | (Value::Number(n), Value::Interval(min, max)) => {
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

            _ => Err(Self::err("Invalid types for multiplication".to_string())),
        }
    }

    pub fn div(&self, other: &Value) -> Result<Value, Error> {
        if let (Value::Unknown, _) | (_, Value::Unknown) = (self, other) {
            return Ok(Value::Unknown);
        }

        if self == other {
            match self {
                Value::Number(n) if *n != 0.0 => return Ok(Value::Number(1.0)),
                Value::Interval(min, max) if *min > 0.0 || *max < 0.0 => return Ok(Value::Number(1.0)),
                _ => {} 
            }
        }

        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 { return Err(Self::err("Division by zero!".to_string())); }
                Ok(Value::Number(a / b))
            }
            (Value::Interval(a_min, a_max), Value::Number(b)) => {
                if *b == 0.0 { return Err(Self::err("Division by zero!".to_string())); }
                let vals = [a_min / b, a_max / b];
                Ok(Value::Interval(
                    vals.iter().copied().fold(f64::INFINITY, f64::min),
                    vals.iter().copied().fold(f64::NEG_INFINITY, f64::max),
                ))
            }
            (Value::Number(a), Value::Interval(b_min, b_max)) => {
                if *b_min <= 0.0 && *b_max >= 0.0 {
                    return Err(Self::err("Division by interval containing zero".to_string()));
                }
                let vals = [a / b_min, a / b_max];
                Ok(Value::Interval(
                    vals.iter().copied().fold(f64::INFINITY, f64::min),
                    vals.iter().copied().fold(f64::NEG_INFINITY, f64::max),
                ))
            }
            (Value::Interval(a_min, a_max), Value::Interval(b_min, b_max)) => {
                if *b_min <= 0.0 && *b_max >= 0.0 {
                    return Err(Self::err("Division by interval containing zero".to_string()));
                }

                let b_recip_min = 1.0 / b_max;
                let b_recip_max = 1.0 / b_min;

                let ips = [
                    a_min * b_recip_min,
                    a_min * b_recip_max,
                    a_max * b_recip_min,
                    a_max * b_recip_max,
                ];

                Ok(Value::Interval(
                    ips.iter().copied().fold(f64::INFINITY, f64::min),
                    ips.iter().copied().fold(f64::NEG_INFINITY, f64::max),
                ))
            }
            _ => Err(Self::err("Division not supported for these types".to_string())),
        }
    }

    pub fn pow(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(*b))),

            (Value::Interval(min, max), Value::Number(n)) => {
                let p1 = min.powf(*n);
                let p2 = max.powf(*n);
                let mut low = p1.min(p2);
                let high = p1.max(p2);
                if n % 2.0 == 0.0 && *min <= 0.0 && *max >= 0.0 { 
                    low = 0.0; 
                }
                Ok(Value::Interval(low, high))
            },
            _ => Err(Self::err("Invalid types for exponentiation".to_string())),
        }
    }

    pub fn modulo(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 {
                    return Err(Self::err("Modulo by zero!".to_string()));
                }
                Ok(Value::Number(a % b))
            }
            _ => Err(Self::err("Invalid types for modulo".to_string())),
        }
    }

    pub fn compare(&self, other: &Value, op: &Token) -> Result<Value, Error> {
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
                _ => Err(Self::err("Invalid comparison for strings".to_string())),
            },

            (Value::Interval(min, max), Value::Number(n)) => 
                Value::Interval(*min, *max).compare(&Value::Interval(*n, *n), op),

            (Value::Number(n), Value::Interval(min, max)) => 
                Value::Interval(*n, *n).compare(&Value::Interval(*min, *max), op),

            _ => Ok(Value::Bool(SKBool::Partial)),
        }
    }

    pub fn logic(&self, other: &Value, op: &Token) -> Result<Value, Error> {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => match op {
                Token::And => Ok(Value::Bool(logic::and(a.clone(), b.clone()))),
                Token::Or => Ok(Value::Bool(logic::or(a.clone(), b.clone()))),
                _ => Err(Self::err("Invalid logic operator".to_string())),
            },
            _ => Err(Self::err("Logic operations require booleans".to_string())),
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
            Value::Symbolic { expression, .. } => write!(f, "{}", Self::format_expr(expression)),
            Value::Unknown => write!(f, "unknown"),
            Value::NativeFn(_) => write!(f, "<native fn>"),
            Value::Function(_) => write!(f, "<function>"),
            Value::Module(_) => write!(f, "<module>"),
            Value::None => write!(f, "none"),
        }
    }
}