use crate::parser::ast::{Expr, IfPolicy, Stmt};
use crate::parser::lexer::{Token, TokenSpan};
use crate::core::value::{Function, SKBool, Value};
use crate::core::logic;
use crate::core::error::Error;
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

    pub fn evaluate(&mut self, statements: Vec<Stmt>) -> Result<Value, Error> {
        let mut last_value = Value::None;
        for stmt in statements {
            last_value = self.eval_stmt(stmt)?;
        }
        Ok(last_value)
    }

    pub fn evaluate_expression(&mut self, expr: Expr) -> Result<Value, Error> {
        self.eval_expr(expr)
    }

    fn execute_block(&mut self, statements: Vec<Stmt>, env: Environment) -> Result<Value, Error> {
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
                    match self.eval_expr(expression) {
                        Ok(val) => last_value = val,
                        Err(e) => {
                            self.env = previous;
                            return Err(e);
                        }
                    }
                }
                _ => {
                    if let Err(e) = self.eval_stmt(stmt) {
                        self.env = previous;
                        return Err(e);
                    }
                    last_value = Value::None;
                }
            }
        }

        self.env = previous;
        Ok(last_value)
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Result<Value, Error> {
        match stmt {
            Stmt::Import { path } => {
                match &path.token {
                    // Case 1: import identifier
                    Token::Identifier(lib_name) => {
                        let registry = crate::libs::get_library_registry();
                        if let Some(register_fn) = registry.get(lib_name) {
                            let mut lib_env = Environment::new(); 
                            register_fn(&mut lib_env);

                            self.env.borrow_mut().define(
                                lib_name.clone(), 
                                Value::Module(Rc::new(RefCell::new(lib_env)))
                            );
                        } else {
                            return Err(Error {
                                token: path.clone(),
                                message: format!("Unknown native library '{}'", lib_name),
                            });
                        }
                    }

                    // Case 2: import "utils.sk"
                    Token::String(file_path) => {
                        let mut final_path = std::path::PathBuf::from(file_path);

                        if !final_path.exists() {
                            let examples_path = std::path::Path::new("examples").join(file_path);
                            if examples_path.exists() {
                                final_path = examples_path;
                            }
                        }

                        let source = std::fs::read_to_string(&final_path).map_err(|e| Error {
                            token: path.clone(),
                            message: format!("Could not find file '{}': {}", file_path, e),
                        })?;

                        let mut lexer = crate::parser::lexer::Lexer::new(source);
                        let tokens = lexer.tokenize().map_err(|msg| Error { 
                            token: path.clone(), 
                            message: msg 
                        })?;
                        
                        let mut parser = crate::parser::parser::Parser::new(tokens);
                        let statements = parser.parse()?;

                        let module_env = Rc::new(RefCell::new(Environment::new()));
                        let mut module_evaluator = Evaluator::new(module_env.clone());
                        module_evaluator.evaluate(statements)?;

                        let module_name = final_path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("module")
                            .to_string();

                        self.env.borrow_mut().define(module_name, Value::Module(module_env));
                    }

                    _ => {
                        return Err(Error {
                            token: path,
                            message: "Import expects a library name or a string path.".to_string(),
                        });
                    }
                }
                Ok(Value::None)
            }
            Stmt::Block { statements } => {
                let new_env = Environment::new_enclosed(self.env.clone());
                self.execute_block(statements, new_env)
            }
            Stmt::Let { name, initializer } => {
                let val = self.eval_expr(initializer)?;
                if let Token::Identifier(n) = &name.token {
                    self.env.borrow_mut().define(n.clone(), val);
                }
                Ok(Value::None)
            }
            Stmt::Symbolic { name, initializer, is_quiet } => {
                if let Token::Identifier(n) = &name.token {
                    self.env.borrow_mut().define(n.clone(), Value::Symbolic {
                        expression: Box::new(initializer),
                        is_quiet,
                    });
                }
                Ok(Value::None)
            }
            Stmt::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                if let Token::Identifier(n) = &name.token {
                    if let Err(msg) = self.env.borrow_mut().assign(n, val) {
                        return Err(Error { token: name, message: msg });
                    }
                }
                Ok(Value::None)
            }
            Stmt::Print { expression } => {
                let val = self.eval_expr(expression)?;
                self.print_value(val);
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
            Stmt::Function { name, params, body } => {
                let function = Value::Function(Function { params, body, closure: self.env.clone() });
                self.env.borrow_mut().define(name.token_to_string(), function);
                Ok(Value::None)
            }
        }
    }

    fn print_value(&mut self, val: Value) {
        match val {
            Value::Symbolic { ref expression, is_quiet } => {
                if is_quiet {
                    if let Ok(resolved) = self.evaluate_expression(*expression.clone()) {
                        println!("{}", resolved);
                    } else {
                        println!("Error resolving quiet symbolic");
                    }
                } else {
                    println!("{}", self.format_symbolic(expression));
                }
            }
            _ => println!("{}", val),
        }
    }

    fn eval_if_chain(
        &mut self,
        cond_expr: Expr,
        body: Stmt,
        remaining_elifs: &[(Expr, Stmt)],
        else_branch: &Option<Box<Stmt>>,
        policy: IfPolicy,
    ) -> Result<Value, Error> {
        let cond_val = self.eval_expr(cond_expr)?;
        let sk_bool = match cond_val {
            Value::Bool(b) => b,
            _ => {
                return Err(Error {
                    token: TokenSpan { token: Token::Unknown, line: 0, column: 0 },
                    message: "Condition must be a boolean".to_string(),
                });
            }
        };

        match sk_bool {
            SKBool::True => self.eval_stmt(body),
            SKBool::False => self.eval_next_in_chain(remaining_elifs, else_branch, policy),
            SKBool::Partial => match policy {
                IfPolicy::Strict => self.eval_next_in_chain(remaining_elifs, &None, policy),
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
        policy: IfPolicy,
    ) -> Result<Value, Error> {
        if let Some(((next_cond, next_body), rest)) = elifs.split_first() {
            self.eval_if_chain(next_cond.clone(), next_body.clone(), rest, else_branch, policy)
        } else if let Some(eb) = else_branch {
            self.eval_stmt(*eb.clone())
        } else {
            Ok(Value::None)
        }
    }

    // Not fully implemented
    fn merge_values(&mut self, v1: Value, v2: Value) -> Result<Value, Error> {
        match (v1, v2) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Interval(n1.min(n2), n1.max(n2))),
            (Value::Interval(l1, h1), Value::Interval(l2, h2)) => Ok(Value::Interval(l1.min(l2), h1.max(h2))),
            (Value::Number(n), Value::Interval(l, h)) | (Value::Interval(l, h), Value::Number(n)) => {
                Ok(Value::Interval(n.min(l), n.max(h)))
            }
            (a, b) if a == b => Ok(a),
            _ => Ok(Value::Unknown),
        }
    }

    fn format_symbolic(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => {
                let l = self.format_symbolic(left);
                let r = self.format_symbolic(right);
                let op = match operator.token {
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
            Expr::Literal { value } => match &value.token {
                Token::Number(n) => format!("{}", n),
                Token::Unknown => "unknown".to_string(),
                Token::String(s) => s.clone(),
                Token::True => "true".to_string(),
                Token::False => "false".to_string(),
                Token::Partial => "partial".to_string(),
                _ => format!("{:?}", value.token),
            },
            Expr::Variable { name } => {
                if let Token::Identifier(n) = &name.token {
                    n.clone()
                } else {
                    format!("{:?}", name.token)
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

    fn eval_expr(&mut self, expr: Expr) -> Result<Value, Error> {
        match expr {
            Expr::Block { statements } => {
                let new_env = Environment::new_enclosed(self.env.clone());
                self.execute_block(statements, new_env)
            }

            Expr::Literal { value } => match value.token {
                Token::Number(n) => Ok(Value::Number(n)),
                Token::String(s) => Ok(Value::String(s)),
                Token::True => Ok(Value::Bool(SKBool::True)),
                Token::False => Ok(Value::Bool(SKBool::False)),
                Token::Partial => Ok(Value::Bool(SKBool::Partial)),
                Token::Unknown => Ok(Value::Unknown),
                Token::None => Ok(Value::None),
                _ => Err(Error {
                    token: value,
                    message: "Unsupported literal".to_string(),
                }),
            },

            Expr::Variable { name } => {
                let name_str = match &name.token {
                    Token::Identifier(n) => n,
                    Token::Print => "print",
                    Token::Input => "input",
                    Token::Kind => "kind",
                    Token::Certain => "certain",
                    Token::Known => "known",
                    Token::Possible => "possible",
                    Token::Impossible => "impossible",
                    Token::Str => "str",
                    Token::Num => "num",
                    Token::Width => "width",
                    Token::Mid => "mid",
                    Token::Intersect => "intersect",
                    Token::Union => "union",
                    _ => {
                        return Err(Error {
                            token: name,
                            message: "Expected identifier".to_string(),
                        })
                    }
                };
                self.env.borrow().get(name_str).map_err(|msg| Error {
                    token: name,
                    message: msg,
                })
            }

            Expr::Interval { min, max, bracket } => {
                let low = self.eval_expr(*min)?;
                let high = self.eval_expr(*max)?;
                match (low, high) {
                    (Value::Number(l), Value::Number(h)) => Ok(Value::Interval(l, h)),
                    _ => Err(Error {
                        token: bracket,
                        message: "Interval bounds must be numbers".to_string(),
                    }),
                }
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let l_val = self.eval_expr(*left)?;
                let r_val = self.eval_expr(*right)?;
                self.apply_binary(l_val, operator, r_val)
            }

            Expr::Unary { operator, right } => {
                let val = self.eval_expr(*right)?;
                match (operator.token.clone(), val) {
                    (Token::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                    (Token::Bang, Value::Bool(b)) => Ok(Value::Bool(logic::not(b))),
                    _ => Err(Error {
                        token: operator,
                        message: "Invalid unary operation".to_string(),
                    }),
                }
            }

            Expr::Grouping { expression } => self.eval_expr(*expression),
            Expr::Call { callee, arguments, paren } => {
                let callee_val = self.eval_expr(*callee)?;

                let mut eval_args = Vec::new();
                for arg in &arguments {
                    eval_args.push(self.eval_expr(arg.clone())?);
                }

                match callee_val {
                    Value::NativeFn(func) => {
                        match func(eval_args, self) {
                            Ok(v) => Ok(v),
                            Err(mut e) => {
                                if matches!(e.token.token, Token::Unknown) {
                                    e.token = paren;
                                }
                                Err(e)
                            }
                        }
                    },
                    Value::Function(func) => {
                        let mut call_env = Environment::new_enclosed(func.closure.clone());

                        for (i, param) in func.params.iter().enumerate() {
                            let value = if i < eval_args.len() {
                                eval_args[i].clone()
                            } else if let Some(default_expr) = &param.default {
                                self.eval_expr(default_expr.clone())?
                            } else {
                                return Err(Error {
                                    token: paren.clone(),
                                    message: format!("Missing required argument '{}'", param.name.token_to_string()),
                                });
                            };

                            call_env.define(param.name.token_to_string(), value);
                        }

                        if eval_args.len() > func.params.len() {
                            return Err(Error {
                                token: paren,
                                message: format!("Expected at most {} args, got {}", func.params.len(), eval_args.len()),
                            });
                        }

                        self.execute_block(func.body.clone(), call_env)
                    }
                    _ => Err(Error {
                        token: paren,
                        message: format!("Value '{}' is not callable", callee_val),
                    }),
                }
            }

            Expr::Get { object, name } => {
                let obj_value = self.eval_expr(*object)?;
                if let Value::Module(mod_env) = obj_value {
                    let member_name = match &name.token {
                        Token::Identifier(s) => s,
                        _ => unreachable!(),
                    };
                    
                    mod_env.borrow().get(member_name).map_err(|msg| Error {
                        token: name.clone(), 
                        message: msg,
                    })
                } else {
                    Err(Error {
                        token: name.clone(),
                        message: "Only modules have properties.".to_string(),
                    })
                }
            }
        }
    }

    fn apply_binary(&mut self, left: Value, op: TokenSpan, right: Value) -> Result<Value, Error> {
        let operator = op.token.clone();

        match operator { // Pre-calculations for x - x, x / x and x * 0
            Token::Star => {
                if let Value::Number(n) = left { if n == 0.0 { return Ok(Value::Number(0.0)); } }
                if let Value::Number(n) = right { if n == 0.0 { return Ok(Value::Number(0.0)); } }
            }
            Token::Minus => {
                if left == right { return Ok(Value::Number(0.0)); }
            }
            Token::Slash => {
                if left == right {
                    match left {
                        Value::Number(n) if n != 0.0 => return Ok(Value::Number(1.0)),
                        Value::Unknown | Value::Symbolic { .. } => return Ok(Value::Number(1.0)),
                        _ => {} 
                    }
                }
            }
            _ => {}
        }

        if left == Value::Unknown || right == Value::Unknown {
            return Ok(Value::Unknown);
        }

        let is_symbolic = left.is_symbolic_or_unknown() || right.is_symbolic_or_unknown();

        let res: Result<Value, String> = match operator {
            Token::Plus => left.add(&right).map_err(|e| e.message),
            Token::Minus => left.sub(&right).map_err(|e| e.message),
            Token::Star => left.mul(&right).map_err(|e| e.message),
            Token::Slash => left.div(&right).map_err(|e| e.message),
            Token::Caret => left.pow(&right).map_err(|e| e.message),

            Token::EqualEqual
            | Token::BangEqual
            | Token::Greater
            | Token::GreaterEqual
            | Token::Less
            | Token::LessEqual => left.compare(&right, &operator).map_err(|e| e.message),

            Token::And | Token::Or => left.logic(&right, &operator).map_err(|e| e.message),

            _ => Err(format!("Unknown binary operator {:?}", operator)),
        };

        match res {
            Ok(val) => Ok(val),
            Err(_) if is_symbolic => self.propagate_symbolic(left, op, right),
            Err(msg) => Err(Error {
                token: op,
                message: msg,
            }),
        }
    }

    fn propagate_symbolic(
        &self,
        left: Value,
        op: TokenSpan,
        right: Value,
    ) -> Result<Value, Error> {
        let is_quiet = match (&left, &right) {
            (Value::Symbolic { is_quiet: q, .. }, _) => *q,
            (_, Value::Symbolic { is_quiet: q, .. }) => *q,
            _ => false,
        };

        let dummy_span = |t: Token| TokenSpan {
            token: t,
            line: 0,
            column: 0,
        };

        Ok(Value::Symbolic {
            expression: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: dummy_span(self.value_to_token(left)),
                }),
                operator: op,
                right: Box::new(Expr::Literal {
                    value: dummy_span(self.value_to_token(right)),
                }),
            }),
            is_quiet,
        })
    }
}