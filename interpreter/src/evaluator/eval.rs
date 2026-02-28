use crate::parser::ast::{Expr, IfPolicy, Stmt, UnitExpr};
use crate::parser::lexer::{Token, TokenSpan};
use crate::core::value::{Function, SKBool, Value};
use crate::core::logic;
use crate::core::units::Unit;
use crate::core::error::{Error, ErrorReporter};
use crate::evaluator::env::Environment;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
pub enum ControlFlow {
    None, Break, Continue,
}

pub struct Evaluator {
    pub env: Rc<RefCell<Environment>>,
    control_flow: ControlFlow,
    reporter: Rc<RefCell<ErrorReporter>>,
    safe_mode: bool,
}

impl Evaluator {
    pub fn new(
        env: Rc<RefCell<Environment>>,
        reporter: Rc<RefCell<ErrorReporter>>,
        safe_mode: bool,
    ) -> Self {
        Self { 
            env,
            control_flow: ControlFlow::None,
            reporter,
            safe_mode,
        }
    }

    pub fn is_safe_mode(&self) -> bool {
        self.safe_mode
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

    pub fn error(&self, token: TokenSpan, msg: impl Into<String>) -> Error {
        self.reporter.borrow_mut().error(token, msg)
    }

    pub fn warn(&self, token: TokenSpan, msg: impl Into<String>) {
        self.reporter.borrow_mut().warn(token, msg);
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
            Stmt::Import { path, alias } => {
                match &path.token {
                    // Case 1: import identifier
                    Token::Identifier(lib_name) => {
                        let registry = crate::libs::get_library_registry();
                        if let Some(register_fn) = registry.get(lib_name) {
                            let mut lib_env = Environment::new(); 
                            register_fn(&mut lib_env);

                            let name = if let Some(a) = &alias {
                                a.token_to_string()
                            } else {
                                lib_name.clone()
                            };

                            self.env.borrow_mut().define(
                                name, 
                                Value::Module(Rc::new(RefCell::new(lib_env)))
                            );
                        } else {
                            return Err(self.report_error(
                                path.clone(),
                                format!("Unknown native library '{}'", lib_name),
                            ));
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

                        let source = std::fs::read_to_string(&final_path).map_err(|e| {
                            self.report_error(
                                path.clone(),
                                format!("Could not find file '{}': {}", file_path, e),
                            )
                        })?;

                        let previous = self.reporter.borrow_mut().set_source(
                            final_path.display().to_string(),
                            source.clone(),
                        );

                        let result = (|| {
                            let mut lexer = crate::parser::lexer::Lexer::new(
                                source,
                                self.reporter.clone(),
                            );
                            let tokens = lexer.tokenize()?;
                            
                            let mut parser = crate::parser::parser::Parser::new(
                                tokens,
                                self.reporter.clone(),
                            );
                            let statements = parser.parse()?;

                            let module_env = Rc::new(RefCell::new(Environment::new()));
                            let mut module_evaluator = Evaluator::new(
                                module_env.clone(),
                                self.reporter.clone(),
                                self.safe_mode,
                            );
                            module_evaluator.evaluate(statements)?;

                            Ok::<_, Error>(module_env)
                        })();

                        self.reporter.borrow_mut().restore_source(previous);

                        let module_env = result?;

                        let module_name = if let Some(a) = &alias {
                            a.token_to_string()
                        } else {
                            final_path
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("module")
                                .to_string()
                        };

                        self.env.borrow_mut().define(module_name, Value::Module(module_env));
                    }

                    _ => {
                        return Err(self.report_error(
                            path,
                            "Import expects a library name or a string path",
                        ));
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
                        return Err(self.report_error(name, msg));
                    }
                }
                Ok(Value::None)
            }
            Stmt::Panic => Err(self.report_error(
                TokenSpan {
                    token: Token::Panic,
                    line: 0,
                    column: 0,
                },
                "Program panicked!",
            )),
            Stmt::Expression { expression } => self.eval_expr(expression),
            Stmt::If { condition, policy, then_branch, elif_branch, else_branch } => {
                self.eval_if_chain(condition, *then_branch, &elif_branch, &else_branch, policy)
            }
            Stmt::TryCatch { try_block, catch_block } => {
                match self.eval_stmt(*try_block) {
                    Ok(val) => Ok(val),
                    Err(_) => self.eval_stmt(*catch_block.clone()),
                }
            }
            Stmt::Function { name, params, body, is_public } => {
                let function = Value::Function(Function { params, body, closure: self.env.clone(), is_public });
                self.env.borrow_mut().define(name.token_to_string(), function);
                Ok(Value::None)
            }
            Stmt::Match { value, arms } => {
                let value = self.eval_expr(value)?;
                for (pattern, body) in arms {
                    if matches!(
                        pattern,
                        Expr::Literal {
                            value: TokenSpan {
                                token: Token::Any,
                                ..
                            },
                        }
                    ) {
                        return self.eval_stmt(body);
                    }

                    if self.eval_expr(pattern)? == value {
                        return self.eval_stmt(body);
                    }
                }
                Ok(Value::None)
            }
            Stmt::Loop { body } => {
                loop {
                    self.control_flow = ControlFlow::None;
                    let new_env = Environment::new_enclosed(self.env.clone());
                    let previous = self.env.clone();
                    self.env = Rc::new(RefCell::new(new_env));

                    for stmt in body.clone() {
                        if let Err(e) = self.eval_stmt(stmt) {
                            self.env = previous.clone();
                            return Err(e);
                        }
                        
                        if self.control_flow == ControlFlow::Break {
                            self.control_flow = ControlFlow::None;
                            self.env = previous.clone();
                            return Ok(Value::None);
                        }
                        
                        if self.control_flow == ControlFlow::Continue {
                            break;
                        }
                    }
                    
                    self.env = previous;
                    
                    if self.control_flow == ControlFlow::Continue {
                        self.control_flow = ControlFlow::None;
                    }
                }
            }
            Stmt::For { variable, iterable, body } => {
                let iterable_val = self.eval_expr(iterable)?;
                
                let items = match iterable_val {
                    Value::Array(arr) => arr,
                    Value::Interval(min, max) => {
                        let mut arr = Vec::new();
                        let mut current = min.ceil();
                        while current <= max {
                            arr.push(Value::Number(current));
                            current += 1.0;
                        }
                        arr
                    }
                    _ => {
                        return Err(self.report_error(
                            variable,
                            "For loop requires an array or interval to iterate".to_string(),
                        ));
                    }
                };

                let var_name = variable.token_to_string();

                for item in items {
                    self.control_flow = ControlFlow::None;
                    let mut new_env = Environment::new_enclosed(self.env.clone());
                    new_env.define(var_name.clone(), item);

                    let previous = self.env.clone();
                    self.env = Rc::new(RefCell::new(new_env));

                    for stmt in body.clone() {
                        if let Err(e) = self.eval_stmt(stmt) {
                            self.env = previous.clone();
                            return Err(e);
                        }

                        if self.control_flow == ControlFlow::Break {
                            self.control_flow = ControlFlow::None;
                            self.env = previous.clone();
                            return Ok(Value::None);
                        }

                        if self.control_flow == ControlFlow::Continue {
                            break;
                        }
                    }

                    self.env = previous;

                    if self.control_flow == ControlFlow::Continue {
                        self.control_flow = ControlFlow::None;
                    }
                }

                Ok(Value::None)
            }
            Stmt::Break => {
                self.control_flow = ControlFlow::Break;
                Ok(Value::None)
            }
            Stmt::Continue => {
                self.control_flow = ControlFlow::Continue;
                Ok(Value::None)
            }
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
                return Err(self.report_error(
                    TokenSpan {
                        token: Token::Unknown,
                        line: 0,
                        column: 0,
                    },
                    "Condition must be a boolean",
                ));
            }
        };

        match sk_bool {
            SKBool::True => self.eval_stmt(body),
            SKBool::False => self.eval_next_in_chain(remaining_elifs, else_branch, policy),
            SKBool::Partial => match policy {
                IfPolicy::Strict => self.eval_next_in_chain(remaining_elifs, &None, policy),
                IfPolicy::Panic => {
                    return Err(self.report_error(
                        TokenSpan {
                            token: Token::Unknown,
                            line: 0,
                            column: 0,
                        },
                        "Program panicked!"
                    ));
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
                _ => Err(self.report_error(value, "Unsupported literal")),
            },

            Expr::Variable { name } => {
                let name_str = match &name.token {
                    Token::Identifier(n) => n,
                    _ => return Err(self.report_error(name, "Expected identifier")),
                };
                let value = self
                    .env
                    .borrow()
                    .get(name_str)
                    .map_err(|msg| self.report_error(name.clone(), msg))?;

                match value {
                    Value::Symbolic { expression, is_quiet: true } => {
                        self.evaluate_expression(*expression)
                    }
                    _ => Ok(value),
                }
            }

            Expr::Postfix { name, operator } => {
                let name_str = match &name.token {
                    Token::Identifier(n) => n.clone(),
                    _ => return Err(self.report_error(name, "Expected identifier")),
                };

                let current = self
                    .env
                    .borrow()
                    .get(&name_str)
                    .map_err(|msg| self.report_error(name.clone(), msg))?;

                let one = Value::Number(1.0);
                let updated = if current == Value::Unknown {
                    Value::Unknown
                } else if current.is_symbolic_or_unknown() {
                    let op_token = match operator.token {
                        Token::Increment => Token::Plus,
                        Token::Decrement => Token::Minus,
                        _ => {
                            return Err(self.report_error(
                                operator,
                                "Invalid postfix operator",
                            ))
                        }
                    };
                    let op_span = TokenSpan {
                        token: op_token,
                        line: operator.line,
                        column: operator.column,
                    };
                    self.propagate_symbolic(current, op_span, one)?
                } else {
                    match operator.token {
                        Token::Increment => current
                            .add(&one)
                            .map_err(|e| self.report_error(operator.clone(), e.message))?,
                        Token::Decrement => current
                            .sub(&one)
                            .map_err(|e| self.report_error(operator.clone(), e.message))?,
                        _ => {
                            return Err(self.report_error(
                                operator,
                                "Invalid postfix operator",
                            ))
                        }
                    }
                };

                if let Err(msg) = self.env.borrow_mut().assign(&name_str, updated.clone()) {
                    return Err(self.report_error(name, msg));
                }

                Ok(updated)
            }

            Expr::Array { elements, bracket: _ } => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.eval_expr(elem)?);
                }
                Ok(Value::Array(values))
            }

            Expr::Index { object, index, bracket } => {
                let arr = self.eval_expr(*object)?;
                let idx = self.eval_expr(*index)?;

                match (arr, idx) {
                    (Value::Array(items), Value::Number(n)) => {
                        let i = n as usize;
                        if i < items.len() {
                            Ok(items[i].clone())
                        } else {
                            Err(self.report_error(bracket, "Array index out of bounds"))
                        }
                    }
                    (Value::Array(items), Value::Interval(min, max)) => {
                        let min_idx = (min as usize).min(items.len());
                        let max_idx = (max as usize).min(items.len());

                        if min_idx >= max_idx {
                            return Ok(Value::Array(Vec::new()));
                        }

                        if min < 0.0 || max >= items.len() as f64 {
                            return Err(self.report_error(
                                bracket,
                                format!(
                                    "Interval index [{}..{}] out of bounds for array of length {}",
                                    min as i32, max as i32, items.len()
                                ),
                            ));
                        }

                        let slice = items[min_idx..=max_idx].to_vec();
                        Ok(Value::Array(slice))
                    }
                    _ => Err(self.report_error(bracket, "Can only index arrays with numbers"))
                }
            }

            Expr::Interval { min, max, bracket } => {
                let low = self.eval_expr(*min)?;
                let high = self.eval_expr(*max)?;
                match (low, high) {
                    (Value::Number(l), Value::Number(h)) => Ok(Value::Interval(l, h)),
                    _ => Err(self.report_error(
                        bracket,
                        "Interval bounds must be numbers",
                    )),
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
                    (Token::Minus, Value::Quantity { value, unit }) => {
                        let negated = Value::Number(0.0).sub(value.as_ref())
                            .map_err(|e| self.report_error(operator.clone(), e.message))?;
                        Ok(Value::Quantity { value: Box::new(negated), unit })
                    }
                    (Token::Bang, Value::Bool(b)) => Ok(Value::Bool(logic::not(b))),
                    _ => Err(self.report_error(operator, "Invalid unary operation")),
                }
            }

            Expr::Grouping { expression } => self.eval_expr(*expression),
            Expr::Quantity { value, unit } => {
                let inner = self.eval_expr(*value)?;
                let (parsed_unit, scale) = self.eval_unit_value(&unit)?;
                let scaled_inner = if scale == 1.0 {
                    inner
                } else {
                    inner
                        .mul(&Value::Number(scale))
                        .map_err(|e| self.report_error(self.unit_token(&unit), e.message))?
                };
                Ok(Value::Quantity {
                    value: Box::new(scaled_inner),
                    unit: parsed_unit,
                })
            }
            Expr::Call { callee, arguments, paren } => {
                if let Expr::Get { object, name } = callee.as_ref() {
                    let mut obj = self.eval_expr(*object.clone())?;
                    let method_name = match &name.token {
                        Token::Identifier(n) => n,
                        _ => return Err(self.report_error(name.clone(), "Expected method name")),
                    };

                    let result = match method_name.as_str() {
                        "len" => {
                            if !arguments.is_empty() {
                                return Err(self.report_error(paren, "len() takes no arguments"));
                            }
                            obj.len()
                                .map_err(|e| self.report_error(name.clone(), e.message))
                        }
                        "push" => {
                            if arguments.is_empty() {
                                return Err(self.report_error(paren, "push() requires at least one argument"));
                            }
                            for arg in arguments {
                                let val = self.eval_expr(arg)?;
                                obj.push(val)
                                    .map_err(|e| self.report_error(name.clone(), e.message))?;
                            }
                            Ok(obj.clone())
                        }
                        "pop" => {
                            if !arguments.is_empty() {
                                return Err(self.report_error(paren, "pop() takes no arguments"));
                            }
                            obj.pop()
                                .map_err(|e| self.report_error(name.clone(), e.message))
                        }
                        "reverse" => {
                            if !arguments.is_empty() {
                                return Err(self.report_error(paren, "reverse() takes no arguments"));
                            }
                            obj.reverse()
                                .map_err(|e| self.report_error(name.clone(), e.message))?;
                            Ok(obj.clone())
                        }
                        _ => {
                            let callee_val = self.eval_expr(*callee.clone())?;
                            return self.call_function(callee_val, arguments, paren);
                        }
                    };

                    if let Expr::Variable { name: var_name } = object.as_ref() {
                        let final_result = result?;
                        if method_name == "push" || method_name == "reverse" {
                            if let Token::Identifier(n) = &var_name.token {
                                self.env.borrow_mut().assign(n, final_result.clone())
                                    .map_err(|msg| self.report_error(var_name.clone(), msg))?;
                            }
                        }
                        Ok(final_result)
                    } else {
                        result
                    }
                } else {
                    // Basic function call
                    let callee_val = self.eval_expr(*callee)?;
                    self.call_function(callee_val, arguments, paren)
                }
            }

            Expr::Get { object, name } => {
                let obj_value = self.eval_expr(*object)?;
                if let Value::Module(mod_env) = obj_value {
                    let member_name = match &name.token {
                        Token::Identifier(s) => s,
                        _ => unreachable!(),
                    };
                    
                    let val = mod_env
                        .borrow()
                        .get(member_name)
                        .map_err(|msg| self.report_error(name.clone(), msg))?;

                    // Check if private
                    if let Value::Function(func) = &val {
                        if !func.is_public {
                            return Err(self.report_error(
                                name.clone(),
                                format!("Function '{}' is private!", member_name),
                            ));
                        }
                    }

                    Ok(val)
                } else {
                    Err(self.report_error(
                        name.clone(),
                        "Only modules have properties!",
                    ))
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
            Token::Modulo => left.modulo(&right).map_err(|e| e.message),

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
            Err(msg) => Err(self.report_error(op, msg)),
        }
    }

    fn eval_unit_value(&self, unit: &UnitExpr) -> Result<(Unit, f64), Error> {
        match unit {
            UnitExpr::Name(name) => {
                let unit_name = name.token_to_string();
                if let Ok(value) = self.env.borrow().get(&unit_name) {
                    return self.unit_from_value(value, name.clone());
                }

                if let Ok(Value::Module(units_mod)) = self.env.borrow().get("units") {
                    if let Ok(value) = units_mod.borrow().get(&unit_name) {
                        return self.unit_from_value(value, name.clone());
                    }
                }

                Err(self.report_error(
                    name.clone(),
                    format!("Undefined unit '{}'", unit_name),
                ))
            }
            UnitExpr::Mul(left, right) => {
                let (l_unit, l_scale) = self.eval_unit_value(left)?;
                let (r_unit, r_scale) = self.eval_unit_value(right)?;
                Ok((l_unit.mul(&r_unit), l_scale * r_scale))
            }
            UnitExpr::Div(left, right) => {
                let (l_unit, l_scale) = self.eval_unit_value(left)?;
                let (r_unit, r_scale) = self.eval_unit_value(right)?;
                Ok((l_unit.div(&r_unit), l_scale / r_scale))
            }
            UnitExpr::Pow(base, exp) => {
                let (base_unit, base_scale) = self.eval_unit_value(base)?;
                Ok((base_unit.pow(*exp), base_scale.powi(*exp)))
            }
        }
    }

    fn unit_from_value(&self, value: Value, name: TokenSpan) -> Result<(Unit, f64), Error> {
        match value {
            Value::Quantity { value, unit } => match value.as_ref() {
                Value::Number(n) => Ok((unit, *n)),
                _ => Err(self.report_error(name, "Unit definitions must be numeric")),
            },
            _ => Err(self.report_error(name, "Unit definitions must be quantities")),
        }
    }

    fn unit_token(&self, unit: &UnitExpr) -> TokenSpan {
        match unit {
            UnitExpr::Name(name) => name.clone(),
            UnitExpr::Mul(left, _) => self.unit_token(left),
            UnitExpr::Div(left, _) => self.unit_token(left),
            UnitExpr::Pow(base, _) => self.unit_token(base),
        }
    }

    fn simplify_symbolic(expr: Expr) -> Expr {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left = Self::simplify_symbolic(*left);
                let right = Self::simplify_symbolic(*right);

                match (&left, &operator.token, &right) {
                    (Expr::Literal { value: l_val }, Token::Plus, Expr::Literal { value: r_val }) => {
                        if let (Token::Number(a), Token::Number(b)) = (&l_val.token, &r_val.token) {
                            return Expr::Literal {
                                value: TokenSpan {
                                    token: Token::Number(a + b),
                                    ..operator.clone()
                                }
                            };
                        }
                    }
                    // x + 0 = x ; x - 0 = x
                    (Expr::Literal { value: val }, Token::Plus, _) | (_, Token::Plus, Expr::Literal { value: val }) => {
                        if let Token::Number(0.0) = val.token {
                            return if let Token::Number(0.0) = val.token { 
                                if matches!(left, Expr::Literal { .. }) { right } else { left } 
                            } else { 
                                Expr::Binary { left: Box::new(left), operator, right: Box::new(right) }
                            };
                        }
                    }
                    // 0 * x = 0 ; 1 * x = x
                    (Expr::Literal { value: val }, Token::Star, _) | (_, Token::Star, Expr::Literal { value: val }) => {
                        if let Token::Number(0.0) = val.token {
                            return Expr::Literal { value: val.clone() };
                        }
                        if let Token::Number(1.0) = val.token {
                            return if matches!(left, Expr::Literal { .. }) { right } else { left };
                        }
                    }
                    _ => {}
                }
                Expr::Binary { left: Box::new(left), operator, right: Box::new(right) }
            }
            _ => expr,
        }
    }

    // Now it should properly handle symbolic values extracting the inner expression
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

        let left_expr = match left {
            Value::Symbolic { expression, .. } => *expression,
            _ => Expr::Literal {
                value: TokenSpan {
                    token: self.value_to_token(left),
                    line: 0,
                    column: 0,
                },
            },
        };

        let right_expr = match right {
            Value::Symbolic { expression, .. } => *expression,
            _ => Expr::Literal {
                value: TokenSpan {
                    token: self.value_to_token(right),
                    line: 0,
                    column: 0,
                },
            },
        };

        let expression = Self::simplify_symbolic(Expr::Binary {
            left: Box::new(left_expr),
            operator: op,
            right: Box::new(right_expr),
        });

        Ok(Value::Symbolic {
            expression: Box::new(expression),
            is_quiet,
        })
    }

    fn report_error(&self, token: TokenSpan, msg: impl Into<String>) -> Error {
        self.error(token, msg)
    }

    fn call_function(&mut self, callee_val: Value, arguments: Vec<Expr>, paren: TokenSpan) -> Result<Value, Error> {
        let mut eval_args = Vec::new();
        for arg in &arguments {
            eval_args.push(self.eval_expr(arg.clone())?);
        }

        match callee_val {
            Value::NativeFn(func) => {
                match func(eval_args, paren.clone(), self) {
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
                        return Err(self.report_error(
                            paren.clone(),
                            format!(
                                "Missing required argument '{}'",
                                param.name.token_to_string()
                            ),
                        ));
                    };

                    call_env.define(param.name.token_to_string(), value);
                }

                if eval_args.len() > func.params.len() {
                    return Err(self.report_error(
                        paren,
                        format!(
                            "Expected at most {} args, got {}",
                            func.params.len(),
                            eval_args.len()
                        ),
                    ));
                }

                self.execute_block(func.body.clone(), call_env)
            }
            _ => Err(self.report_error(
                paren,
                format!("Value '{}' is not callable", callee_val),
            )),
        }
    }
}