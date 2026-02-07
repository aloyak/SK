use crate::parser::lexer::{Token, TokenSpan};
use crate::parser::ast::{Expr, IfPolicy, Parameter, Stmt};
use crate::core::error::{Error, ErrorKind, ErrorReporter};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Parser {
    tokens: Vec<TokenSpan>,
    current: usize,
    reporter: Rc<RefCell<ErrorReporter>>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenSpan>, reporter: Rc<RefCell<ErrorReporter>>) -> Self {
        Self {
            tokens,
            current: 0,
            reporter,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if self.match_token(Token::NewLine) {
                continue;
            }
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn skip_newlines(&mut self) {
        while self.check(&Token::NewLine) {
            self.advance();
        }
    }

    // --- Declarations ---

    fn declaration(&mut self) -> Result<Stmt, Error> {
        match self.peek().token {
            Token::Import => {
                self.advance();
                self.import_declaration()
            }
            Token::Public => {
                self.advance();
                self.consume(Token::Function, "Expected 'fn' after 'pub'")?;
                self.function_declaration(true)
            }
            Token::Let => {
                self.advance();
                self.let_declaration()
            }
            Token::Symbolic => {
                self.advance();
                self.symbolic_declaration(false)
            }
            Token::Quiet => {
                self.advance();
                self.symbolic_declaration(true)
            }
            Token::Unknown => {
                self.advance();
                self.unknown_declaration()
            }
            Token::If => {
                self.advance();
                self.if_statement()
            },
            Token::Panic => {
                self.advance();
                self.panic_statement()
            },
            Token::Loop => {
                self.advance();
                self.loop_statement()
            }
            Token::Break => {
                self.advance();
                self.end_stmt()?;
                Ok(Stmt::Break)
            }
            Token::Continue => {
                self.advance();
                self.end_stmt()?;
                Ok(Stmt::Continue)
            }
            _ => self.statement(),
        }
    }
    
    fn import_declaration(&mut self) -> Result<Stmt, Error> {
        let peeked = self.peek().token.clone();
        
        match peeked {
            Token::Identifier(_) | Token::String(_) => {
                let path = self.advance().clone();
                
                let alias = if self.match_token(Token::As) {
                    Some(self.consume_identifier("Expect alias name after 'as'")?)
                } else {
                    None
                };

                Ok(Stmt::Import { path, alias })
            }
            _ => Err(self.report_error(
                self.peek().clone(),
                "Expect library name or file path after 'import'",
            )),
        }
    }

    fn loop_statement(&mut self) -> Result<Stmt, Error> {
        self.skip_newlines();
        self.consume(Token::LBrace, "Expect '{' before loop body")?;
        
        let body = self.block()?;
        Ok(Stmt::Loop { body })
    }

    fn function_declaration(&mut self, is_public: bool) -> Result<Stmt, Error> {
        let name = self.consume_identifier("Expect function name")?;

        self.consume(Token::LParen, "Expect '(' after function name")?;
        
        let mut parameters = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                let param_name = self.consume_identifier("Expect parameter name")?;
                let mut default = None;

                if self.match_token(Token::Assign) {
                    default = Some(self.expression()?);
                }

                parameters.push(Parameter { name: param_name, default });
                if !self.match_token(Token::Comma) { break; }
            }
        }
        
        self.consume(Token::RParen, "Expect ')' after parameters")?;
        
        self.skip_newlines();
        self.consume(Token::LBrace, "Expect '{' before function body")?;
        
        let body = self.block()?; 

        Ok(Stmt::Function {
            name,
            params: parameters,
            body,
            is_public,
        })
    }

    fn symbolic_declaration(&mut self, is_quiet: bool) -> Result<Stmt, Error> {
        let name = self.consume_identifier("Expect variable name")?;
        self.consume(Token::Assign, "Expect '=' after name")?;
        let initializer = self.expression()?;
        self.end_stmt()?;
        Ok(Stmt::Symbolic { name, initializer, is_quiet })
    }

    fn unknown_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.consume_identifier("Expect variable name after 'unknown'")?;
        self.end_stmt()?;
        
        let unknown_span = TokenSpan { 
            token: Token::Unknown, 
            line: name.line, 
            column: name.column 
        };

        Ok(Stmt::Let { 
            name, 
            initializer: Expr::Literal { value: unknown_span } 
        })
    }

    fn let_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.consume_identifier("Expect variable name")?;
        self.consume(Token::Assign, "Expect '=' after variable name")?;
        
        let initializer = self.expression()?; 
        
        self.end_stmt()?;
        Ok(Stmt::Let { name, initializer })
    }

    
    fn block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();
        
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            if self.match_token(Token::NewLine) {
                continue;
            }
            statements.push(self.declaration()?);
        }
        
        self.consume(Token::RBrace, "Expect '}' after block")?;
        Ok(statements)
    }
    
    
    // --- Statements ---
    
    fn statement(&mut self) -> Result<Stmt, Error> {
        if self.match_token(Token::Function) {
            return self.function_declaration(false)
        }

        if self.match_token(Token::LBrace) {
            return Ok(Stmt::Block { statements: self.block()? });
        }
        
        if self.peek_type(Token::Identifier(String::new())) && self.peek_next_type(Token::Assign) {
            let name = self.advance().clone(); 
            self.advance(); // consume '='
            let value = self.expression()?;
            self.end_stmt()?;
            return Ok(Stmt::Assign { name, value });
        }
        let expr = self.expression()?;
        self.end_stmt()?;
        Ok(Stmt::Expression { expression: expr })
    }
        
    fn if_statement(&mut self) -> Result<Stmt, Error> {
        let condition = self.expression()?;
        
        let policy = if self.match_token(Token::Arrow) {
            match self.advance().token {
                Token::Strict => IfPolicy::Strict,
                Token::Merge => IfPolicy::Merge,
                Token::Panic => IfPolicy::Panic,
                _ => {
                    return Err(self.report_error(
                        self.previous().clone(),
                        "Expected policy (strict, merge, panic) after '->'",
                    ))
                }
            }
        } else {
            IfPolicy::Strict // Default to strict policy
        };

        self.skip_newlines();
        let then_branch = Box::new(self.statement()?);
        let mut elif_branch = Vec::new();

        self.skip_newlines();

        while self.match_token(Token::Elif) {
            let elif_cond = self.expression()?;
            
            self.skip_newlines(); 
            let elif_body = self.statement()?;
            elif_branch.push((elif_cond, elif_body));

            self.skip_newlines();
        }

        let mut else_branch = None;
        if self.match_token(Token::Else) {
            self.skip_newlines();
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::If {
            condition,
            policy,
            then_branch,
            elif_branch,
            else_branch,
        })
    }

    fn panic_statement(&mut self) -> Result<Stmt, Error> {
        self.end_stmt()?;
        Ok(Stmt::Panic)
    }


    pub fn expression(&mut self) -> Result<Expr, Error> {
        self.logic_or()
    }

    // Helper method to parse binary operators
    fn binary(&mut self, next: fn(&mut Self) -> Result<Expr, Error>, operators: &[Token]) -> Result<Expr, Error> {
        let mut expr = next(self)?;
        while self.match_any(operators) {
            let operator = self.previous().clone();
            let right = next(self)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, Error> {
        self.binary(Self::multiplication, &[Token::Plus, Token::Minus])
    }

    fn logic_or(&mut self) -> Result<Expr, Error> {
        self.binary(Self::logic_and, &[Token::Or])
    }

    fn logic_and(&mut self) -> Result<Expr, Error> {
        self.binary(Self::equality, &[Token::And])
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        self.binary(Self::comparison, &[Token::EqualEqual, Token::BangEqual])
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        self.binary(Self::addition, &[Token::Greater, Token::GreaterEqual, Token::Less, Token::LessEqual])
    }

    fn multiplication(&mut self) -> Result<Expr, Error> {
        self.binary(Self::power, &[Token::Star, Token::Slash])
    }

    fn power(&mut self) -> Result<Expr, Error> {
        self.binary(Self::unary, &[Token::Caret])
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_any(&[Token::Bang, Token::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(Token::LParen) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(Token::Dot) {
                let name = self.consume_identifier("Expect property name after '.'")?;
                expr = Expr::Get { object: Box::new(expr), name };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
        let mut arguments = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.match_token(Token::Comma) { break; }
            }
        }
        let paren = self.consume(Token::RParen, "Expect ')' after arguments")?.clone();

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.match_token(Token::True) { return Ok(Expr::Literal { value: self.previous().clone() }); }
        if self.match_token(Token::False) { return Ok(Expr::Literal { value: self.previous().clone() }); }
        if self.match_token(Token::Partial) { return Ok(Expr::Literal { value: self.previous().clone() }); }
        if self.match_token(Token::None) { return Ok(Expr::Literal { value: self.previous().clone() }); }
        if self.match_token(Token::Unknown) { return Ok(Expr::Literal { value: self.previous().clone() }); }

        if self.match_token(Token::LBrace) {
            let statements = self.block()?;
            return Ok(Expr::Block { statements });
        }

        if self.match_any(&[Token::Number(0.0), Token::String("".to_string())]) {
            return Ok(Expr::Literal { value: self.previous().clone() });
        }

        if self.match_token(Token::Identifier("".to_string())) 
            || self.match_token(Token::Print) 
            || self.match_token(Token::Input)
            || self.match_token(Token::Kind)
            || self.match_token(Token::Certain)
            || self.match_token(Token::Known)
            || self.match_token(Token::Possible)
            || self.match_token(Token::Impossible)
            || self.match_token(Token::String("".to_string())) {

            return Ok(Expr::Variable { name: self.previous().clone() });
        }

        if self.match_token(Token::LParen) {
            let expr = self.expression()?;
            self.consume(Token::RParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping { expression: Box::new(expr) });
        }

        if self.match_token(Token::LBracket) {
            let min = self.expression()?;
            self.consume(Token::RangeSep, "Expect '..' in interval")?;
            let max = self.expression()?;
            let bracket = self.consume(Token::RBracket, "Expect ']' after interval")?.clone();
            return Ok(Expr::Interval { min: Box::new(min), max: Box::new(max), bracket });
        }

        Err(self.report_error(self.peek().clone(), "Expect expression"))
    }

    fn end_stmt(&mut self) -> Result<(), Error> {
        if self.is_at_end() { return Ok(()); }
        if self.match_token(Token::NewLine) { return Ok(()); }
        if self.check(&Token::RBrace) { return Ok(()); }
        Err(self.report_error(
            self.peek().clone(),
            "Expect newline or EOF after statement",
        ))
    }

    fn match_token(&mut self, t: Token) -> bool {
        if self.check(&t) {
            self.advance();
            return true;
        }
        false
    }

    fn match_any(&mut self, types: &[Token]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, t: &Token) -> bool {
        if self.is_at_end() { return false; }
        std::mem::discriminant(&self.peek().token) == std::mem::discriminant(t)
    }

    fn advance(&mut self) -> &TokenSpan {
        if !self.is_at_end() { self.current += 1; }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token == Token::EOF
    }

    fn peek(&self) -> &TokenSpan {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &TokenSpan {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, t: Token, msg: &str) -> Result<&TokenSpan, Error> {
        if self.check(&t) { return Ok(self.advance()); }
        Err(self.report_error(self.peek().clone(), msg))
    }

    fn peek_type(&self, t: Token) -> bool {
        if self.is_at_end() { return false; }
        std::mem::discriminant(&self.peek().token) == std::mem::discriminant(&t)
    }

    fn peek_next_type(&self, t: Token) -> bool {
        if self.current + 1 >= self.tokens.len() { return false; }
        std::mem::discriminant(&self.tokens[self.current + 1].token) == std::mem::discriminant(&t)
    }

    fn consume_identifier(&mut self, msg: &str) -> Result<TokenSpan, Error> {
        let t = self.peek().clone();
        match t.token {
            Token::Identifier(_) => {
                self.advance();
                Ok(t)
            },
            _ => Err(self.report_error(t, msg)),
        }
    }

    fn report_error(&self, token: TokenSpan, msg: &str) -> Error {
        self.reporter
            .borrow_mut()
            .error_with_kind(ErrorKind::Syntax, token, msg)
    }
}