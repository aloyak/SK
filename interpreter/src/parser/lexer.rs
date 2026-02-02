#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    UnknownChar(char),
    Identifier(String),

    Number(f64),
    String(String),

    Import,

    // Keywords
    Let,
    Const,
    Unknown,
    Symbolic,
    Quiet,
    If,
    Elif,
    Else,
    Merge,
    Strict,
    None,
    Print,
    Input,
    Str,
    Num,
    Panic,
    Function,
    Return,
    For,
    While,
    Kind,
    Comma,
    Dot,

    // Operators & Symbols
    Assign,
    Arrow,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    RangeSep,
    
    // Delimiters
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Quote,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    
    EqualEqual,
    BangEqual,
    
    And,
    Or,
    Bang,

    // Knowledge Operators
    Possible,
    Impossible,
    Certain,
    Known,

    // Interval Operators
    Width, // max - min
    Mid,   // midpoint
    Intersect, // returns overlapping of two intervals!
    Union,     // returns the smallest interval that contains both

    True,
    False,
    Partial,

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
}

pub fn tokenize(raw: String) -> Result<Vec<TokenSpan>, String> {
    let mut lexer = Lexer::new(raw);
    lexer.tokenize()
}

pub struct Lexer {
    source: Vec<char>,
    cursor: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            source: input.chars().collect(),
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<TokenSpan>, String> {
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

    fn next_token(&mut self) -> Result<Option<Token>, String> {
        let c = self.advance();

        match c {
            '(' => Ok(Some(Token::LParen)),
            ')' => Ok(Some(Token::RParen)),
            '[' => Ok(Some(Token::LBracket)),
            ']' => Ok(Some(Token::RBracket)),
            '{' => Ok(Some(Token::LBrace)),
            '}' => Ok(Some(Token::RBrace)),
            ',' => Ok(Some(Token::Comma)),
            '+' => Ok(Some(Token::Plus)),
            '*' => Ok(Some(Token::Star)),
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
            '-' => {
                if self.match_char('>') { Ok(Some(Token::Arrow)) } 
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
                else { Err("Expected '&' after '&'".to_string()) }
            }
            '|' => {
                if self.match_char('|') { Ok(Some(Token::Or)) }
                else { Err("Expected '|' after '|'".to_string()) }
            }

            // Whitespace
            ' ' | '\r' | '\t' => Ok(None), 

            '"' | '\'' => self.string(c).map(Some),

            _ => {
                if c.is_ascii_digit() {
                    Ok(Some(self.number()))
                } else if c.is_alphabetic() || c == '_' {
                    Ok(Some(self.identifier()))
                } else {
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
            "let" => Token::Let,
            "const" => Token::Const,
            "unknown" => Token::Unknown,
            "symbolic" => Token::Symbolic,
            "quiet" => Token::Quiet,
            "if" => Token::If,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "merge" => Token::Merge,
            "strict" => Token::Strict,
            "panic!" => Token::Panic, // The statement
            "panic" => Token::Panic,  // The policy
            "print" => Token::Print,
            "input" => Token::Input,
            "fn" => Token::Function,
            "return" => Token::Return,
            "for" => Token::For,
            "while" => Token::While,
            "none" => Token::None,
            "kind" => Token::Kind,
            "true" => Token::True,
            "false" => Token::False,
            "partial" => Token::Partial,
            "possible" => Token::Possible,
            "impossible" => Token::Impossible,
            "certain" => Token::Certain,
            "known" => Token::Known,
            "width" => Token::Width,
            "mid" => Token::Mid,
            "intersection" => Token::Intersect,
            "union" => Token::Union,
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

    fn string(&mut self, quote_type: char) -> Result<Token, String> {
        let mut text = String::new();

        while self.peek() != quote_type && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            text.push(self.advance());
        }

        if self.is_at_end() {
            return Err(format!("Unterminated string at line {}", self.line));
        }

        self.advance();

        Ok(Token::String(text))
    }
}