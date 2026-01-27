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

    fn declaration(&mut self) -> Result<Stmt, String> { // change this to a match later (symbolic, quiet, const, etc...)
        if self.match_token(Token::Let) {
            self.let_declaration()

        } else if self.match_token(Token::Unknown) {
            self.unknown_declaration()
            
        } else {
            self.statement()
        }
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
        if self.match_token(Token::Print) {
            let expr = self.expression()?;
            self.end_stmt()?;
            return Ok(Stmt::Print { expression: expr });
        }

        let expr = self.expression()?;
        self.end_stmt()?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.match_any(&[Token::Equal, Token::NotEqual]) {
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
        let mut expr = self.term()?;
        while self.match_any(&[Token::Greater, Token::GreaterEqual, Token::Less, Token::LessEqual]) {
            let operator = self.previous().token.clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.match_any(&[Token::Plus, Token::Minus]) {
            let operator = self.previous().token.clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_any(&[Token::Star, Token::Slash]) {
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
        if self.match_any(&[Token::Not, Token::Minus]) {
            let operator = self.previous().token.clone();
            let right = self.unary()?;
            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(Token::True) { return Ok(Expr::Literal { value: Token::True }); }
        if self.match_token(Token::False) { return Ok(Expr::Literal { value: Token::False }); }
        if self.match_token(Token::None) { return Ok(Expr::Literal { value: Token::None }); }
        if self.match_token(Token::Unknown) { return Ok(Expr::Literal { value: Token::Unknown }); }

        let token_span = self.advance();
        match &token_span.token {
            Token::Number(_) | Token::String(_) => {
                Ok(Expr::Literal { value: token_span.token.clone() })
            }
            Token::Identifier(_) => {
                Ok(Expr::Variable { name: token_span.token.clone() })
            }
            Token::LParen => {
                let expr = self.expression()?;
                self.consume(Token::RParen, "Expect ')' after expression.")?;
                Ok(Expr::Grouping { expression: Box::new(expr) })
            }
            Token::LBracket => {
                let min = self.expression()?;
                self.consume(Token::RangeSep, "Expect '..' in interval.")?;
                let max = self.expression()?;
                self.consume(Token::RBracket, "Expect ']' after interval.")?;
                Ok(Expr::Interval { 
                    min: Box::new(min), 
                    max: Box::new(max) 
                })
            }
            _ => Err(format!("Expect expression at line {}, col {}", token_span.line, token_span.column)),
        }
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

    fn consume(&mut self, t: Token, msg: &str) -> Result<&TokenSpan, String> {
        if self.check(&t) { return Ok(self.advance()); }
        Err(format!("{} found {:?} at line {}", msg, self.peek().token, self.peek().line))
    }

    fn consume_identifier(&mut self, msg: &str) -> Result<Token, String> {
        let t = self.advance().token.clone();
        match t {
            Token::Identifier(_) => Ok(t),
            _ => Err(format!("{} found {:?} at line {}", msg, t, self.previous().line)),
        }
    }

    fn end_stmt(&mut self) -> Result<(), String> {
        if !self.is_at_end() && !self.match_token(Token::NewLine) {
            return Err(format!("Expect newline after statement at line {}", self.previous().line));
        }
        Ok(())
    }
}