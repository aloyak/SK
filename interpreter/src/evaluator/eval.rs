use crate::parser::ast::{Expr, IfPolicy, Stmt};
use crate::parser::lexer::Token;
use crate::core::value::{Value, SKBool};
use crate::core::logic;
use crate::evaluator::env::Environment;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new(env: Rc<RefCell<Environment>>) -> Self {
        Self { env }
    }

    pub fn evaluate(&mut self, statements: Vec<Stmt>) -> Result<Value, String> {
        let mut last_value = Value::None;
        for stmt in statements {
            last_value = self.eval_stmt(stmt)?;
        }
        Ok(last_value)
    }

    fn execute_block(&mut self, statements: Vec<Stmt>, env: Environment) -> Result<Value, String> {
        let previous = self.env.clone();
        self.env = Rc::new(RefCell::new(env));

        let mut last_value = Value::None;
        let len = statements.len();

        // Use into_iter to take ownership of statements
        for (i, stmt) in statements.into_iter().enumerate() {
            let is_last = i == len - 1;

            match stmt {
                // Only a bare expression on the last line can a value.
                Stmt::Expression { expression } if is_last => {
                    last_value = self.eval_expr(expression)?;
                }
                // Or return none
                _ => {
                    self.eval_stmt(stmt)?;
                    last_value = Value::None;
                }
            }
        }

        self.env = previous;
        Ok(last_value)
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Block { statements } => {
                let new_env = Environment::new_enclosed(self.env.clone());
                self.execute_block(statements, new_env)
            }
            Stmt::Let { name, initializer } => {
                let val = self.eval_expr(initializer)?;
                if let Token::Identifier(n) = name {
                    self.env.borrow_mut().define(n, val);
                }
                Ok(Value::None)
            }
            Stmt::Symbolic { name, initializer, is_quiet } => {
                if let Token::Identifier(n) = name {
                    self.env.borrow_mut().define(n, Value::Symbolic { 
                        expression: Box::new(initializer), 
                        is_quiet 
                    });
                }
                Ok(Value::None)
            }
            Stmt::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                if let Token::Identifier(n) = name {
                    self.env.borrow_mut().assign(&n, val)?;
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
            Stmt::If { condition, policy, then_branch, else_branch } => {
                let cond_val = self.eval_expr(condition)?;
                let sk_bool = match cond_val {
                    Value::Bool(b) => b,
                    _ => return Err("If condition must be a boolean".to_string()),
                };

                match sk_bool {
                    SKBool::True => self.eval_stmt(*then_branch),
                    SKBool::False => {
                        if let Some(eb) = else_branch {
                            self.eval_stmt(*eb)
                        } else {
                            Ok(Value::None)
                        }
                    }
                    SKBool::Partial => match policy {
                        IfPolicy::Strict => Ok(Value::None),
                        IfPolicy::Panic => {
                            eprintln!("Program panicked! Uncertain condition with panic policy.");
                            std::process::exit(1);
                        }
                        IfPolicy::Merge => {
                            let val_then = self.eval_stmt(*then_branch)?;
                            let val_else = if let Some(eb) = else_branch {
                                self.eval_stmt(*eb)?
                            } else {
                                Value::None
                            };

                            match (val_then, val_else) {
                                (Value::Number(n1), Value::Number(n2)) => 
                                    Ok(Value::Interval(n1.min(n2), n1.max(n2))),
                                (Value::Interval(l1, h1), Value::Interval(l2, h2)) => 
                                    Ok(Value::Interval(l1.min(l2), h1.max(h2))),
                                (Value::Number(n), Value::Interval(l, h)) | (Value::Interval(l, h), Value::Number(n)) =>
                                    Ok(Value::Interval(n.min(l), n.max(h))),
                                (v1, v2) if v1 == v2 => Ok(v1),
                                _ => Ok(Value::Unknown),
                            }
                        }
                    }
                }
            }
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
            Expr::Block { .. } => "{...}".to_string(),
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
                Token::Input => Some("input".to_string()),
                Token::Kind => Some("kind".to_string()),
                Token::Certain => Some("certain".to_string()),
                Token::Known => Some("known".to_string()),
                Token::Possible => Some("possible".to_string()),
                Token::Impossible => Some("impossible".to_string()),
                Token::Str => Some("str".to_string()),
                Token::Num => Some("num".to_string()),
                Token::Width => Some("width".to_string()),
                Token::Mid => Some("mid".to_string()),
                Token::Intersect => Some("intersect".to_string()),
                Token::Union => Some("union".to_string()),

                _ => None,
            },
            Expr::Grouping { expression } => self.get_func_name(expression),
            _ => None,
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Block { statements } => {
                let new_env = Environment::new_enclosed(self.env.clone());
                self.execute_block(statements, new_env)
            }

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
                    self.env.borrow().get(&n)
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
                    "num" => {
                        if eval_args.len() != 1 { return Err("num() expects 1 arg".to_string()); }

                        match &eval_args[0] {
                            Value::String(s) => {
                                s.parse::<f64>()
                                    .map(Value::Number)
                                    .map_err(|_| format!("Conversion Error: Cannot parse '{}' as a number", s))
                            }
                            Value::Number(n) => Ok(Value::Number(*n)),
                            _ => Err("num() argument must be a string or number".to_string()),
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
                    "input" => {
                        use std::io::{self, Write};
                        if eval_args.len() > 1 { return Err("input() expects 0 or 1 arg".to_string()); }

                        if let Some(prompt) = eval_args.get(0) {
                            print!("{}", prompt);
                            io::stdout().flush().map_err(|e| e.to_string())?;
                        }

                        let mut buffer = String::new();
                        io::stdin().read_line(&mut buffer).map_err(|e| e.to_string())?;

                        Ok(Value::String(buffer.trim_end().to_string()))
                    }
                    "certain" => {
                        if eval_args.len() != 1 { return Err("certain() expects 1 arg".to_string()); }
                        let res = match eval_args[0] {
                            Value::Bool(SKBool::True) => SKBool::True,
                            _ => SKBool::False,
                        };
                        Ok(Value::Bool(res))
                    }
                    "possible" => {
                        if eval_args.len() != 1 { return Err("possible() expects 1 arg".to_string()); }
                        let res = match eval_args[0] {
                            Value::Bool(SKBool::False) => SKBool::False,
                            _ => SKBool::True,
                        };
                        Ok(Value::Bool(res))
                    }
                    "impossible" => {
                        if eval_args.len() != 1 { return Err("impossible() expects 1 arg".to_string()); }
                        let res = match eval_args[0] {
                            Value::Bool(SKBool::False) => SKBool::True,
                            _ => SKBool::False,
                        };
                        Ok(Value::Bool(res))
                    }
                    "known" => {
                        if eval_args.len() != 1 { return Err("known() expects 1 arg".to_string()); }
                        let res = match eval_args[0] {
                            Value::Bool(SKBool::Partial) | 
                            Value::Unknown | 
                            Value::Interval(_, _) | 
                            Value::Symbolic { .. } => SKBool::False,
                            _ => SKBool::True,
                        };
                        Ok(Value::Bool(res))
                    }
                    "width" => {
                        if eval_args.len() != 1 { return Err("width() expects 1 arg".to_string()); }
                        match eval_args[0] {
                            Value::Interval(min, max) => Ok(Value::Number(max - min)),
                            _ => Err("width() requires an interval".to_string()),
                        }
                    }
                    "mid" => {
                        if eval_args.len() != 1 { return Err("mid() expects 1 arg".to_string()); }
                        match eval_args[0] {
                            Value::Interval(min, max) => Ok(Value::Number((min + max) / 2.0)),
                            _ => Err("mid() requires an interval".to_string()),
                        }
                    }
                    "union" => {
                        if eval_args.len() != 2 { return Err("union() expects 2 args".to_string()); }
                        match (&eval_args[0], &eval_args[1]) {
                            (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
                                Ok(Value::Interval(min1.min(*min2), max1.max(*max2)))
                            }
                            _ => Err("union() requires two intervals".to_string()),
                        }
                    }
                    "intersect" => {
                        if eval_args.len() != 2 { return Err("intersect() expects 2 args".to_string()); }
                        match (&eval_args[0], &eval_args[1]) {
                            (Value::Interval(min1, max1), Value::Interval(min2, max2)) => {
                                let start = min1.max(*min2);
                                let end = max1.min(*max2);
                                if start <= end {
                                    Ok(Value::Interval(start, end))
                                } else {
                                    Ok(Value::None) // No overlap = None
                                }
                            }
                            _ => Err("intersect() requires two intervals".to_string()),
                        }
                    }
                    _ => Err(format!("Unknown function '{}'", func_name)),
                }
            }
        }
    }

    fn apply_binary(&self, left: Value, op: Token, right: Value) -> Result<Value, String> {
        // 1. Handle Symbolic / Unknown propagation immediately
        // Note: 0 * Unknown is handled inside Value::mul optimization, 
        // so we only strictly propagate if concrete calculation isn't possible.
        
        let is_symbolic = left.is_symbolic_or_unknown() || right.is_symbolic_or_unknown();
        
        // We attempt concrete calculation first if it allows for optimizations (like 0 * symbolic)
        // If the types don't match or strictly require symbolic propagation, we fall back.
        
        let res = match op {
            Token::Plus => left.add(&right),
            Token::Minus => left.sub(&right),
            Token::Star => left.mul(&right),
            Token::Slash => left.div(&right),
            Token::Caret => left.pow(&right),
            
            Token::EqualEqual | Token::BangEqual | 
            Token::Greater | Token::GreaterEqual | 
            Token::Less | Token::LessEqual => left.compare(&right, &op),
            
            Token::And | Token::Or => left.logic(&right, &op),
            
            _ => Err(format!("Unknown binary operator {:?}", op)),
        };

        match res {
            Ok(val) => Ok(val),
            Err(_) if is_symbolic => self.propagate_symbolic(left, op, right),
            Err(e) => Err(e),
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