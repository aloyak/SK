use crate::parser::ast::{Expr, IfPolicy, Stmt};
use crate::parser::lexer::Token;
use crate::core::value::{Value, SKBool};
use crate::core::logic;
use crate::evaluator::env::Environment;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Evaluator {
    pub env: Rc<RefCell<Environment>>,
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

    pub fn evaluate_expression(&mut self, expr: Expr) -> Result<Value, String> {
        self.eval_expr(expr)
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
            Stmt::If { condition, policy, then_branch, elif_branch, else_branch } => {
                self.eval_if_chain(condition, *then_branch, &elif_branch, &else_branch, policy)
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

    fn eval_if_chain(
        &mut self,
        cond_expr: Expr,
        body: Stmt,
        remaining_elifs: &[(Expr, Stmt)],
        else_branch: &Option<Box<Stmt>>,
        policy: IfPolicy
    ) -> Result<Value, String> {
        let cond_val = self.eval_expr(cond_expr)?;
        let sk_bool = match cond_val {
            Value::Bool(b) => b,
            _ => return Err("Condition must be a boolean".to_string()),
        };

        match sk_bool {
            SKBool::True => self.eval_stmt(body),
            SKBool::False => self.eval_next_in_chain(remaining_elifs, else_branch, policy),
            SKBool::Partial => match policy {
                IfPolicy::Strict => {
                    self.eval_next_in_chain(remaining_elifs, &None, policy)
                }
                IfPolicy::Panic => {
                    eprintln!("Program panicked! Uncertain condition with panic policy.");
                    std::process::exit(1);
                }
                IfPolicy::Merge => {
                    let val_true = self.eval_stmt(body)?;
                    let val_false = self.eval_next_in_chain(remaining_elifs, else_branch, policy)?;
                    self.merge_values(val_true, val_false)
                }
            },
        }
    }

    fn eval_next_in_chain(
        &mut self,
        elifs: &[(Expr, Stmt)],
        else_branch: &Option<Box<Stmt>>,
        policy: IfPolicy
    ) -> Result<Value, String> {
        if let Some(((next_cond, next_body), rest)) = elifs.split_first() {
            self.eval_if_chain(next_cond.clone(), next_body.clone(), rest, else_branch, policy)
        } else if let Some(eb) = else_branch {
            self.eval_stmt(*eb.clone())
        } else {
            Ok(Value::None)
        }
    }

    // Not fully implemented
    fn merge_values(&mut self, v1: Value, v2: Value) -> Result<Value, String> {
        match (v1, v2) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Interval(n1.min(n2), n1.max(n2))),
            (Value::Interval(l1, h1), Value::Interval(l2, h2)) => Ok(Value::Interval(l1.min(l2), h1.max(h2))),
            (Value::Number(n), Value::Interval(l, h)) | (Value::Interval(l, h), Value::Number(n)) =>
                Ok(Value::Interval(n.min(l), n.max(h))),
            (a, b) if a == b => Ok(a),
            _ => Ok(Value::Unknown),
        }
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
            _ => Token::Unknown,
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
                let name_str = match name {
                    Token::Identifier(n) => n,
                    Token::Print => "print".to_string(),
                    Token::Input => "input".to_string(),
                    Token::Kind => "kind".to_string(),
                    Token::Certain => "certain".to_string(),
                    Token::Known => "known".to_string(),
                    Token::Possible => "possible".to_string(),
                    Token::Impossible => "impossible".to_string(),
                    Token::Str => "str".to_string(),
                    Token::Num => "num".to_string(),
                    Token::Width => "width".to_string(),
                    Token::Mid => "mid".to_string(),
                    Token::Intersect => "intersect".to_string(),
                    Token::Union => "union".to_string(),
                    _ => return Err("Expected identifier".to_string()),
                };
                self.env.borrow().get(&name_str)
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
                let callee_val = self.eval_expr(*callee)?;
                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.eval_expr(arg)?);
                }
                match callee_val {
                    Value::NativeFn(func) => func(args, self),
                    _ => Err(format!("Value '{}' is not callable", callee_val)),
                }
            }
        }
    }

    fn apply_binary(&mut self, left: Value, op: Token, right: Value) -> Result<Value, String> {
        if left == Value::Unknown || right == Value::Unknown {
            return Ok(Value::Unknown);
        }

        let is_symbolic = left.is_symbolic_or_unknown() || right.is_symbolic_or_unknown();

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