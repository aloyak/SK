use std::cell::RefCell;
use std::rc::Rc;

use crate::core::error::{Error, ErrorKind, ErrorReporter};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    UnknownChar(char),
    Identifier(String),

    Number(f64),
    String(String),

    Import,
    As,

    // Keywords
    Let,
    Unknown,
    Symbolic,
    Quiet,
    Public,
    If,
    Elif,
    Else,
    Merge,
    Strict,
    None,
    Panic,
    Function,
    Comma,
    Dot,

    Loop,
    Break,
    Continue,
    For,
    In,

    // Operators & Symbols
    Assign,
    Arrow,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    RangeSep,
    Modulo,
    
    // Delimiters
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    
    EqualEqual,
    BangEqual,
    
    And,
    Or,
    Bang,


    True,
    False,
    Partial,

    Increment, // ++
    Decrement, // --
    AdditionAssign, // +=
    SubtractionAssign, // -=

    NewLine,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenSpan {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

impl TokenSpan {
    pub fn token_to_string(&self) -> String {
        match &self.token {
            Token::Identifier(s) => s.clone(),
            Token::String(s) => s.clone(),
            Token::Number(n) => n.to_string(),
            _ => format!("{:?}", self.token)
        }
    }

    pub fn display_len(&self) -> usize {
        match &self.token {
            Token::Identifier(s) => s.len(),
            Token::String(s) => s.len() + 2,
            Token::Number(n) => n.to_string().len(),
            Token::UnknownChar(_) => 1,
            Token::True => 4,
            Token::False => 5,
            Token::Partial => 7,
            Token::None => 4,
            _ => 1,
        }
    }
}

pub fn tokenize(raw: String, reporter: Rc<RefCell<ErrorReporter>>) -> Result<Vec<TokenSpan>, Error> {
    let mut lexer = Lexer::new(raw, reporter);
    lexer.tokenize()
}

pub struct Lexer {
    source: Vec<char>,
    cursor: usize,
    line: usize,
    column: usize,
    reporter: Rc<RefCell<ErrorReporter>>,
}

impl Lexer {
    pub fn new(input: String, reporter: Rc<RefCell<ErrorReporter>>) -> Self {
        Self {
            source: input.chars().collect(),
            cursor: 0,
            line: 1,
            column: 1,
            reporter,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<TokenSpan>, Error> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let line = self.line;
            let column = self.column;

            if let Some(token) = self.next_token()? {
                tokens.push(TokenSpan {
                    token,
                    line,
                    column,
                });
            }
        }
        
        tokens.push(TokenSpan { 
            token: Token::EOF, 
            line: self.line, 
            column: self.column 
        });
        
        Ok(tokens)
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() { '\0' } else { self.source[self.cursor] }
    }

    fn peek_next(&self) -> char {
        if self.cursor + 1 >= self.source.len() { '\0' } else { self.source[self.cursor + 1] }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.cursor];
        self.cursor += 1;
        self.column += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.cursor] != expected {
            return false;
        }
        self.cursor += 1;
        self.column += 1;
        true
    }

    fn next_token(&mut self) -> Result<Option<Token>, Error> {
        let c = self.advance();
        let start_line = self.line;
        let start_column = self.column.saturating_sub(1);

        match c {
            '(' => Ok(Some(Token::LParen)),
            ')' => Ok(Some(Token::RParen)),
            '[' => Ok(Some(Token::LBracket)),
            ']' => Ok(Some(Token::RBracket)),
            '{' => Ok(Some(Token::LBrace)),
            '}' => Ok(Some(Token::RBrace)),
            ',' => Ok(Some(Token::Comma)),
            '*' => Ok(Some(Token::Star)),
            '%' => Ok(Some(Token::Modulo)),
            '^' => Ok(Some(Token::Caret)),

            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(None)
                } else {
                    Ok(Some(Token::Slash))
                }
            }

            '\n' => {
                self.line += 1;
                self.column = 1;

                // we shouldn't take 2 newlines together as that => empty operation, so we skip it
                while self.peek() == '\n' || self.peek() == '\r' {
                    let next_c = self.advance();
                    if next_c == '\n' {
                        self.line += 1;
                        self.column = 1;
                    }
                }

                Ok(Some(Token::NewLine))
            }

            '=' => {
                if self.match_char('=') { Ok(Some(Token::EqualEqual)) } 
                else { Ok(Some(Token::Assign)) }
            }
            '+' => {
                if self.match_char('+') { Ok(Some(Token::Increment)) }
                else if self.match_char('=') { Ok(Some(Token::AdditionAssign)) }
                else { Ok(Some(Token::Plus)) }
            },
            '-' => {
                if self.match_char('>') { Ok(Some(Token::Arrow)) } 
                else if self.match_char('-') { Ok(Some(Token::Decrement)) }
                else if self.match_char('=') { Ok(Some(Token::SubtractionAssign)) }
                else { Ok(Some(Token::Minus)) }
            }
            '.' => {
                if self.match_char('.') { Ok(Some(Token::RangeSep)) } 

                else { Ok(Some(Token::Dot)) }
            }

            '>' => {
                if self.match_char('=') { Ok(Some(Token::GreaterEqual)) } 
                else { Ok(Some(Token::Greater)) }
            }
            '<' => {
                if self.match_char('=') { Ok(Some(Token::LessEqual)) } 
                else { Ok(Some(Token::Less)) }
            }
            '!' => {
                if self.match_char('=') { Ok(Some(Token::BangEqual)) } 
                else { Ok(Some(Token::Bang)) }
            }
            '&' => {
                if self.match_char('&') { Ok(Some(Token::And)) }
                else { Err(self.error_at(start_line, start_column, "Expected '&' after '&'")) }
            }
            '|' => {
                if self.match_char('|') { Ok(Some(Token::Or)) }
                else { Err(self.error_at(start_line, start_column, "Expected '|' after '|'")) }
            }

            // Whitespace
            ' ' | '\r' | '\t' => Ok(None), 

            '"' | '\'' => self.string(c, start_line, start_column).map(Some),

            _ => {
                if c.is_ascii_digit() {
                    Ok(Some(self.number()))
                } else if c.is_alphabetic() || c == '_' {
                    Ok(Some(self.identifier()))
                } else {
                    self.reporter.borrow_mut().warn(
                        TokenSpan {
                            token: Token::UnknownChar(c),
                            line: start_line,
                            column: start_column,
                        },
                        format!("Unknown character '{}'", c),
                    );
                    Ok(Some(Token::UnknownChar(c)))
                }
            }
        }
    }

    fn identifier(&mut self) -> Token {
        let mut text = String::new();
        text.push(self.source[self.cursor - 1]);

        while self.peek().is_alphanumeric() || self.peek() == '_' {
            text.push(self.advance());
        }

        // special case for "panic!"
        if text == "panic" && self.peek() == '!' {
            text.push(self.advance());
        }

        match text.as_str() {
            "import" => Token::Import,
            "as" => Token::As,
            "let" => Token::Let,
            "unknown" => Token::Unknown,
            "symbolic" => Token::Symbolic,
            "quiet" => Token::Quiet,
            "pub" => Token::Public,
            "if" => Token::If,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "merge" => Token::Merge,
            "strict" => Token::Strict,
            "panic!" => Token::Panic, // The statement
            "panic" => Token::Panic,  // The policy
            "fn" => Token::Function,
            "loop" => Token::Loop,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "for" => Token::For,
            "in" => Token::In,
            "none" => Token::None,
            "true" => Token::True,
            "false" => Token::False,
            "partial" => Token::Partial,
            _ => Token::Identifier(text),
        }
    }

    fn number(&mut self) -> Token {
        let mut text = String::new();
        text.push(self.source[self.cursor - 1]);

        while self.peek().is_ascii_digit() {
            text.push(self.advance());
        }

        // decimal logic
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            text.push(self.advance());
            while self.peek().is_ascii_digit() {
                text.push(self.advance());
            }
        }

        // scientific notation!
        if self.peek() == 'e' || self.peek() == 'E' {
            text.push(self.advance());

            if self.peek() == '-' || self.peek() == '+' {   // allow for 5e-10 or 5e+10
                text.push(self.advance());
            }
            while self.peek().is_ascii_digit() {
                text.push(self.advance());
            }
        }

        let val: f64 = text.parse().unwrap_or(0.0);
        Token::Number(val)
    }

    fn string(&mut self, quote_type: char, start_line: usize, start_column: usize) -> Result<Token, Error> {
            let mut text = String::new();

            while self.peek() != quote_type && !self.is_at_end() {
                if self.peek() == '\n' {
                    self.line += 1;
                    self.column = 1;
                }

                let c = self.advance();
                
                if c == '\\' {
                    match self.peek() {
                        'n' => {
                            text.push('\n');
                            self.advance();
                        }
                        't' => {
                            text.push('\t');
                            self.advance();
                        }
                        'r' => {
                            text.push('\r');
                            self.advance();
                        }
                        '\\' => {
                            text.push('\\');
                            self.advance();
                        }
                        '"' => {
                            text.push('\"');
                            self.advance();
                        }
                        '\'' => {
                            text.push('\'');
                            self.advance();
                        }
                        _ => {
                            text.push('\\');
                        }
                    }
                } else {
                    text.push(c);
                }
            }

            if self.is_at_end() {
                return Err(self.error_at(start_line, start_column, "Unterminated string"));
            }

            self.advance();
            Ok(Token::String(text))
        }

    fn error_at(&self, line: usize, column: usize, msg: &str) -> Error {
        self.reporter
            .borrow_mut()
            .error_with_kind(
                ErrorKind::Syntax,
                TokenSpan {
                    token: Token::None,
                    line,
                    column,
                },
                msg,
            )
    }
}