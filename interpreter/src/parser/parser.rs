use crate::parser::lexer::{Token, TokenSpan};
use crate::parser::ast::{Stmt, Expr};

pub struct Parser {
    tokens: Vec<TokenSpan>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenSpan>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if self.match_token(Token::NewLine) {
                continue;
            }
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        match self.peek().token {
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
            Token::Panic => {
                self.advance();
                self.panic_statement()
            }
            _ => self.statement(),
        }
    }

    fn symbolic_declaration(&mut self, is_quiet: bool) -> Result<Stmt, String> {
        let name = self.consume_identifier("Expect variable name.")?;
        self.consume(Token::Assign, "Expect '=' after name.")?;
        let initializer = self.expression()?;
        self.end_stmt()?;
        Ok(Stmt::Symbolic { name, initializer, is_quiet })
    }

    fn unknown_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume_identifier("Expect variable name after 'unknown'.")?;
        self.end_stmt()?;
        
        Ok(Stmt::Let { 
            name, 
            initializer: Expr::Literal { value: Token::Unknown } 
        })
    }

    fn let_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume_identifier("Expect variable name.")?;
        self.consume(Token::Assign, "Expect '=' after variable name.")?;

        let initializer = self.expression()?;

        self.end_stmt()?;
        Ok(Stmt::Let { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        // Check for assignment: identifier = expression
        if self.peek_type(Token::Identifier(String::new())) && self.peek_next_type(Token::Assign) {
            let name = self.advance().token.clone();
            self.advance(); // consume '='
            let value = self.expression()?;
            self.end_stmt()?;
            return Ok(Stmt::Assign { name, value });
        }

        let expr = self.expression()?;

        if let Expr::Call { ref callee, .. } = expr {
            if let Expr::Variable { name: Token::Identifier(ref n) } = **callee {
                if n == "print" {
                    self.end_stmt()?;
                    return Ok(Stmt::Print { expression: expr });
                }
            }
        }

        if let Expr::Variable { name: Token::Identifier(ref n) } = expr {
            if n == "print" {
                return Err("Syntax Error: 'print' is a function and requires parentheses. Use: print(...)".to_string());
            }
        }

        self.end_stmt()?;
        Ok(Stmt::Expression { expression: expr })
    }

    pub fn expression(&mut self) -> Result<Expr, String> {
        self.logic_or()
    }

    fn addition(&mut self) -> Result<Expr, String> {
        let mut expr = self.multiplication()?;

        while self.match_any(&[Token::Plus, Token::Minus]) {
            let operator = self.previous().token.clone();
            let right = self.multiplication()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn logic_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.logic_and()?;
        while self.match_token(Token::Or) {
            let operator = self.previous().token.clone();
            let right = self.logic_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn logic_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;
        while self.match_token(Token::And) {
            let operator = self.previous().token.clone();
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.match_any(&[Token::EqualEqual, Token::BangEqual]) {
            let operator = self.previous().token.clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.addition()?;
        while self.match_any(&[Token::Greater, Token::GreaterEqual, Token::Less, Token::LessEqual]) {
            let operator = self.previous().token.clone();
            let right = self.addition()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, String> {
        let mut expr = self.power()?;

        while self.match_tokens(&[Token::Star, Token::Slash]) {
            let operator = self.previous().token.clone();
            let right = self.power()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn power(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_token(Token::Caret) {
            let operator = self.previous().token.clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_any(&[Token::Bang, Token::Minus]) {
            let operator = self.previous().token.clone();
            let right = self.unary()?;
            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(Token::LParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.match_token(Token::Comma) { break; }
            }
        }
        self.consume(Token::RParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            arguments,
        })
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(Token::True) { return Ok(Expr::Literal { value: Token::True }); }
        if self.match_token(Token::False) { return Ok(Expr::Literal { value: Token::False }); }
        if self.match_token(Token::Partial) { return Ok(Expr::Literal { value: Token::Partial }); }
        if self.match_token(Token::None) { return Ok(Expr::Literal { value: Token::None }); }
        if self.match_token(Token::Unknown) { return Ok(Expr::Literal { value: Token::Unknown }); }

        if self.match_any(&[Token::Number(0.0), Token::String("".to_string())]) {
            return Ok(Expr::Literal { value: self.previous().token.clone() });
        }

        if self.match_token(Token::Identifier("".to_string())) 
            || self.match_token(Token::Print) 
            || self.match_token(Token::Kind)  
            || self.match_token(Token::String("".to_string())) {

            return Ok(Expr::Variable { name: self.previous().token.clone() }); // Just return, don't call finish_call
        }

        if self.match_token(Token::LParen) {
            let expr = self.expression()?;
            self.consume(Token::RParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping { expression: Box::new(expr) });
        }

        if self.match_token(Token::LBracket) {
            let min = self.expression()?;
            self.consume(Token::RangeSep, "Expect '..' in interval.")?;
            let max = self.expression()?;
            self.consume(Token::RBracket, "Expect ']' after interval.")?;
            return Ok(Expr::Interval { min: Box::new(min), max: Box::new(max) });
        }

        Err(format!("Expect expression, found {:?} at line {}", self.peek().token, self.peek().line))
    }

    fn end_stmt(&mut self) -> Result<(), String> {
        if self.is_at_end() { return Ok(()); }
        if self.match_token(Token::NewLine) { return Ok(()); }
        Err(format!("Expect newline or EOF after statement, found {:?}", self.peek().token))
    }

    fn match_token(&mut self, t: Token) -> bool {
        if self.check(&t) {
            self.advance();
            return true;
        }
        false
    }

    fn match_tokens(&mut self, types: &[Token]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
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

    fn consume(&mut self, t: Token, msg: &str) -> Result<&TokenSpan, String> {
        if self.check(&t) { return Ok(self.advance()); }
        Err(format!("{} found {:?} at line {}", msg, self.peek().token, self.peek().line))
    }

    fn panic_statement(&mut self) -> Result<Stmt, String> {
        self.end_stmt()?;
        Ok(Stmt::Panic)
    }

    fn peek_type(&self, t: Token) -> bool {
        if self.is_at_end() { return false; }
        std::mem::discriminant(&self.peek().token) == std::mem::discriminant(&t)
    }

    fn peek_next_type(&self, t: Token) -> bool {
        if self.current + 1 >= self.tokens.len() { return false; }
        std::mem::discriminant(&self.tokens[self.current + 1].token) == std::mem::discriminant(&t)
    }

    fn consume_identifier(&mut self, msg: &str) -> Result<Token, String> {
        let t = self.peek().token.clone();
        match t {
            Token::Identifier(_) => {
                self.advance();
                Ok(t)
            },
            _ => Err(format!("{} found {:?} at line {}", msg, t, self.peek().line)),
        }
    }
}