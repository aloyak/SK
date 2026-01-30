use crate::parser::ast::{Expr, Stmt};
use crate::parser::lexer::Token;
use crate::core::value::{Value, SKBool};
use crate::core::logic;
use crate::evaluator::env::Environment;

pub struct Evaluator<'a> {
    env: &'a mut Environment,
}

impl<'a> Evaluator<'a> {
    pub fn new(env: &'a mut Environment) -> Self {
        Self { env }
    }

    pub fn evaluate(&mut self, statements: Vec<Stmt>) -> Result<Value, String> {
        let mut last_value = Value::None;
        for stmt in statements {
            last_value = self.eval_stmt(stmt)?;
        }
        Ok(last_value)
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Let { name, initializer } => {
                let val = self.eval_expr(initializer)?;
                if let Token::Identifier(n) = name {
                    self.env.define(n, val);
                }
                Ok(Value::None)
            }
            Stmt::Symbolic { name, initializer, is_quiet } => {
                if let Token::Identifier(n) = name {
                    self.env.define(n, Value::Symbolic { 
                        expression: Box::new(initializer), 
                        is_quiet 
                    });
                }
                Ok(Value::None)
            }
            Stmt::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                if let Token::Identifier(n) = name {
                    self.env.define(n, val);
                }
                Ok(Value::None)
            }
            Stmt::Print { expression } => {
                let val = self.eval_expr(expression)?;
                self.print_value(val)?;
                Ok(Value::None)
            }
            Stmt::Panic => {
                eprintln!("Program panicked!");
                std::process::exit(1);
            }
            Stmt::Expression { expression } => self.eval_expr(expression),
        }
    }

    fn print_value(&mut self, val: Value) -> Result<(), String> {
        match val {
            Value::Symbolic { ref expression, is_quiet } => {
                if is_quiet {
                    let resolved = self.eval_expr(*expression.clone())?;
                    println!("{}", resolved);
                } else {
                    println!("{}", self.format_symbolic(expression));
                }
            }
            _ => println!("{}", val),
        }
        Ok(())
    }

    fn format_symbolic(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => {
                let l = self.format_symbolic(left);
                let r = self.format_symbolic(right);
                let op = match operator {
                    Token::Plus => "+",
                    Token::Minus => "-",
                    Token::Star => "*",
                    Token::Slash => "/",
                    Token::Caret => "^",
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
            Expr::Literal { value } => match value {
                Token::Number(n) => format!("{}", n),
                Token::Unknown => "unknown".to_string(),
                Token::String(s) => s.clone(),
                Token::True => "true".to_string(),
                Token::False => "false".to_string(),
                Token::Partial => "partial".to_string(),
                _ => format!("{:?}", value),
            },
            Expr::Variable { name } => {
                if let Token::Identifier(n) = name {
                    n.clone()
                } else {
                    format!("{:?}", name)
                }
            }
            Expr::Grouping { expression } => format!("({})", self.format_symbolic(expression)),
            _ => "...".to_string(),
        }
    }

    fn value_to_token(&self, value: Value) -> Token {
        match value {
            Value::Number(n) => Token::Number(n),
            Value::String(s) => Token::String(s),
            Value::Bool(SKBool::True) => Token::True,
            Value::Bool(SKBool::False) => Token::False,
            Value::Bool(SKBool::Partial) => Token::Partial,
            Value::Unknown => Token::Unknown,
            Value::None => Token::None,
            Value::Interval(_, _) => Token::Unknown,
            Value::Symbolic { .. } => Token::Unknown,
        }
    }

    fn get_func_name(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Variable { name } => match name {
                Token::Identifier(n) => Some(n.clone()),
                Token::Print => Some("print".to_string()),
                Token::Kind => Some("kind".to_string()),
                _ => None,
            },
            Expr::Grouping { expression } => self.get_func_name(expression),
            _ => None,
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal { value } => match value {
                Token::Number(n) => Ok(Value::Number(n)),
                Token::String(s) => Ok(Value::String(s)),
                Token::True => Ok(Value::Bool(SKBool::True)),
                Token::False => Ok(Value::Bool(SKBool::False)),
                Token::Partial => Ok(Value::Bool(SKBool::Partial)),
                Token::Unknown => Ok(Value::Unknown),
                Token::None => Ok(Value::None),
                _ => Err(format!("Unsupported literal: {:?}", value)),
            }

            Expr::Variable { name } => {
                if let Token::Identifier(n) = name {
                    self.env.get(&n)
                } else {
                    Err(format!("Cannot use keyword '{:?}' as a variable", name))
                }
            }

            Expr::Interval { min, max } => {
                let low = self.eval_expr(*min)?;
                let high = self.eval_expr(*max)?;
                match (low, high) {
                    (Value::Number(l), Value::Number(h)) => Ok(Value::Interval(l, h)),
                    _ => Err("Interval bounds must be numbers".to_string()),
                }
            }

            Expr::Binary { left, operator, right } => {
                let l_val = self.eval_expr(*left)?;
                let r_val = self.eval_expr(*right)?;
                self.apply_binary(l_val, operator, r_val)
            }

            Expr::Unary { operator, right } => {
                let val = self.eval_expr(*right)?;
                match (operator, val) {
                    (Token::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                    (Token::Bang, Value::Bool(b)) => Ok(Value::Bool(logic::not(b))),
                    _ => Err("Invalid unary operation".to_string()),
                }
            }

            Expr::Grouping { expression } => self.eval_expr(*expression),
            Expr::Call { callee, arguments } => {
                let func_name = self.get_func_name(&callee)
                    .ok_or_else(|| format!("Invalid function call: expected identifier, found {:?}", callee))?;
                let mut eval_args = Vec::new();
                for arg in arguments {
                    eval_args.push(self.eval_expr(arg)?);
                }
                match func_name.as_str() {
                    "resolve" => {
                        if eval_args.len() != 1 { return Err("resolve() expects 1 arg".to_string()); }
                        match eval_args[0].clone() {
                            Value::Symbolic { expression, .. } => self.eval_expr(*expression),
                            other => Ok(other),
                        }
                    }
                    "kind" => {
                        if eval_args.len() != 1 { return Err("kind() expects 1 arg".to_string()); }
                        let type_name = match eval_args[0] {
                            Value::Symbolic { is_quiet: true, .. } => "quiet",
                            Value::Symbolic { is_quiet: false, .. } => "symbolic",
                            Value::Number(_) => "number",
                            Value::String(_) => "string",
                            Value::Bool(_) => "bool",
                            Value::Interval(_, _) => "interval",
                            Value::Unknown => "unknown",
                            _ => "none",
                        };
                        Ok(Value::String(type_name.to_string()))
                    }
                    "str" => {
                        if eval_args.len() != 1 { return Err("str() expects 1 arg".to_string()); }
                        match eval_args[0] {
                            Value::Symbolic { ref expression, is_quiet } => {
                                if is_quiet {
                                    let resolved = self.eval_expr(*expression.clone())?;
                                    Ok(Value::String(format!("{}", resolved)))
                                } else {
                                    Ok(Value::String(self.format_symbolic(expression)))
                                }
                            }
                            _ => Ok(Value::String(format!("{}", eval_args[0]))),
                        }
                    }
                    "print" => {
                        for arg in eval_args {
                            match arg {
                                Value::Symbolic { ref expression, is_quiet } => {
                                    if is_quiet {
                                        let resolved = self.eval_expr(*expression.clone())?;
                                        print!("{} ", resolved);
                                    } else {
                                        print!("{} ", self.format_symbolic(expression));
                                    }
                                }
                                _ => print!("{} ", arg),
                            }
                        }
                        println!();
                        Ok(Value::None)
                    }
                    _ => Err(format!("Unknown function '{}'", func_name)),
                }
            }
        }
    }

    fn apply_binary(&self, left: Value, op: Token, right: Value) -> Result<Value, String> {
        match (left.clone(), op.clone(), right.clone()) {
            // Multiplication by Zero: 0 * unknown = 0
            (Value::Number(n), Token::Star, _) if n == 0.0 => Ok(Value::Number(0.0)),
            (_, Token::Star, Value::Number(n)) if n == 0.0 => Ok(Value::Number(0.0)),

            // Self-Subtraction: x - x = 0 (even if x is unknown or an interval)
            (l, Token::Minus, r) if l == r && l != Value::Unknown => Ok(Value::Number(0.0)),
            
            // Division by Self: x / x = 1 (excluding zero/unknown/intervals containing zero)
            (l, Token::Slash, r) if l == r => {
                match l {
                    Value::Number(n) if n != 0.0 => Ok(Value::Number(1.0)),
                    Value::Interval(min, max) if min > 0.0 || max < 0.0 => Ok(Value::Number(1.0)),
                    _ => Ok(Value::Unknown),
                }
            }

            (Value::Number(a), Token::Caret, Value::Number(b)) => Ok(Value::Number(a.powf(b))),

            (Value::Interval(min, max), Token::Caret, Value::Number(n)) => {
                if n % 2.0 == 0.0 {
                    let p1 = min.powf(n);
                    let p2 = max.powf(n);
                    let mut low = p1.min(p2);
                    let high = p1.max(p2);
                    if min <= 0.0 && max >= 0.0 {
                        low = 0.0;
                    }
                    Ok(Value::Interval(low, high))
                } else {
                    let p1 = min.powf(n);
                    let p2 = max.powf(n);
                    Ok(Value::Interval(p1.min(p2), p1.max(p2)))
                }
            }

            (Value::Number(a), Token::Plus, Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Number(a), Token::Minus, Value::Number(b)) => Ok(Value::Number(a - b)),
            (Value::Number(a), Token::Star, Value::Number(b)) => Ok(Value::Number(a * b)),
            (Value::Number(a), Token::Slash, Value::Number(b)) => {
                if b == 0.0 { return Err("Division by zero".to_string()); }
                Ok(Value::Number(a / b))
            }

            // Interval & Number
            (Value::Interval(min, max), Token::Plus, Value::Number(n)) |
            (Value::Number(n), Token::Plus, Value::Interval(min, max)) => Ok(Value::Interval(min + n, max + n)),
            
            (Value::Interval(min, max), Token::Minus, Value::Number(n)) => Ok(Value::Interval(min - n, max - n)),
            (Value::Number(n), Token::Minus, Value::Interval(min, max)) => Ok(Value::Interval(n - max, n - min)),

            (Value::Interval(min, max), Token::Star, Value::Number(n)) |
            (Value::Number(n), Token::Star, Value::Interval(min, max)) => {
                let a = min * n;
                let b = max * n;
                Ok(Value::Interval(a.min(b), a.max(b)))
            },

            (Value::Interval(min1, max1), Token::Plus, Value::Interval(min2, max2)) => {
                Ok(Value::Interval(min1 + min2, max1 + max2))
            },
            (Value::Interval(min1, max1), Token::Minus, Value::Interval(min2, max2)) => {
                Ok(Value::Interval(min1 - max2, max1 - min2))
            },
            (Value::Interval(min1, max1), Token::Star, Value::Interval(min2, max2)) => {
                let p = [min1 * min2, min1 * max2, max1 * min2, max1 * max2];
                Ok(Value::Interval(
                    p.iter().copied().fold(f64::INFINITY, f64::min),
                    p.iter().copied().fold(f64::NEG_INFINITY, f64::max)
                ))
            }

            (Value::Number(a), operator, Value::Number(b)) => {
                let op_str = match operator {
                    Token::EqualEqual => "==",
                    Token::BangEqual => "!=",
                    Token::Greater => ">",
                    Token::GreaterEqual => ">=",
                    Token::Less => "<",
                    Token::LessEqual => "<=",
                    _ => return self.propagate_symbolic(left, op, right),
                };
                Ok(Value::Bool(logic::compare_nums(a, b, op_str)))
            }

            (Value::String(s1), Token::EqualEqual, Value::String(s2)) => {
                Ok(Value::Bool(if s1 == s2 { SKBool::True } else { SKBool::False }))
            }
            (Value::String(s1), Token::BangEqual, Value::String(s2)) => {
                Ok(Value::Bool(if s1 != s2 { SKBool::True } else { SKBool::False }))
            }

            (Value::Interval(min1, max1), operator, Value::Interval(min2, max2)) => {
                let op_str = match operator {
                    Token::Greater => ">",
                    Token::Less => "<",
                    Token::GreaterEqual => ">=",
                    Token::LessEqual => "<=",
                    Token::EqualEqual => "==",
                    Token::BangEqual => "!=",
                    _ => return self.propagate_symbolic(left, op, right),
                };
                Ok(Value::Bool(logic::compare_intervals(min1, max1, min2, max2, op_str)))
            }

            (Value::Interval(min, max), operator, Value::Number(n)) => {
                self.apply_binary(Value::Interval(min, max), operator, Value::Interval(n, n))
            }
            (Value::Number(n), operator, Value::Interval(min, max)) => {
                self.apply_binary(Value::Interval(n, n), operator, Value::Interval(min, max))
            }

            (Value::Bool(a), Token::And, Value::Bool(b)) => Ok(Value::Bool(logic::and(a, b))),
            (Value::Bool(a), Token::Or, Value::Bool(b)) => Ok(Value::Bool(logic::or(a, b))),
            
            (Value::Symbolic { .. }, _, _) | (_, _, Value::Symbolic { .. }) | (Value::Unknown, _, _) | (_, _, Value::Unknown) => {
                self.propagate_symbolic(left, op, right)
            }

            (Value::String(mut s1), Token::Plus, Value::String(s2)) => {
                s1.push_str(&s2);
                Ok(Value::String(s1))
            }

            _ => Err("Operation not supported for these types".to_string()),
        }
    }

    fn propagate_symbolic(&self, left: Value, op: Token, right: Value) -> Result<Value, String> {
        let is_quiet = match (&left, &right) {
            (Value::Symbolic { is_quiet: q, .. }, _) => *q,
            (_, Value::Symbolic { is_quiet: q, .. }) => *q,
            _ => false,
        };

        Ok(Value::Symbolic {
            expression: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal { value: self.value_to_token(left) }),
                operator: op,
                right: Box::new(Expr::Literal { value: self.value_to_token(right) }),
            }),
            is_quiet,
        })
    }
}