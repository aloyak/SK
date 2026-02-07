use std::fmt;

use crate::parser::lexer::{Token, TokenSpan};

const COLOR_RESET: &str = "\x1b[0m";
const COLOR_RED: &str = "\x1b[91m";
const COLOR_BLUE: &str = "\x1b[94m";
const COLOR_YELLOW: &str = "\x1b[93m";
const STYLE_BOLD: &str = "\x1b[1m";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Runtime,
    Syntax,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub token: TokenSpan,
    pub message: String,
    pub kind: ErrorKind,
    pub file: Option<String>,
    pub line_text: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Warning {
    pub token: TokenSpan,
    pub message: String,
    pub file: Option<String>,
    pub line_text: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct ErrorReporter {
    errors: Vec<Error>,
    warnings: Vec<Warning>,
    context: Option<SourceContext>,
}

#[derive(Debug, Clone)]
pub struct SourceContext {
    name: String,
    lines: Vec<String>,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_source(&mut self, name: impl Into<String>, source: impl Into<String>) -> Option<SourceContext> {
        let prev = self.context.take();
        let source = source.into();
        let lines = source.lines().map(|line| line.to_string()).collect();
        self.context = Some(SourceContext {
            name: name.into(),
            lines,
        });
        prev
    }

    pub fn restore_source(&mut self, previous: Option<SourceContext>) {
        self.context = previous;
    }

    pub fn error(&mut self, token: TokenSpan, message: impl Into<String>) -> Error {
        self.error_with_kind(ErrorKind::Runtime, token, message)
    }

    pub fn error_with_kind(
        &mut self,
        kind: ErrorKind,
        token: TokenSpan,
        message: impl Into<String>,
    ) -> Error {
        let (file, line_text) = self.capture_context(&token);
        let err = Error {
            token,
            message: message.into(),
            kind,
            file,
            line_text,
        };
        self.errors.push(err.clone());
        err
    }

    pub fn warn(&mut self, token: TokenSpan, message: impl Into<String>) {
        let (file, line_text) = self.capture_context(&token);
        self.warnings.push(Warning {
            token,
            message: message.into(),
            file,
            line_text,
        });
    }

    pub fn take_warnings(&mut self) -> Vec<Warning> {
        std::mem::take(&mut self.warnings)
    }

    pub fn take_errors(&mut self) -> Vec<Error> {
        std::mem::take(&mut self.errors)
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn capture_context(&self, token: &TokenSpan) -> (Option<String>, Option<String>) {
        let Some(context) = &self.context else {
            return (None, None);
        };

        if token.line == 0 {
            return (Some(context.name.clone()), None);
        }

        let line_idx = token.line.saturating_sub(1);
        let line_text = context.lines.get(line_idx).cloned();
        (Some(context.name.clone()), line_text)
    }
}

impl Error {
    pub fn new(token: TokenSpan, message: impl Into<String>) -> Self {
        Self {
            token,
            message: message.into(),
            kind: ErrorKind::Runtime,
            file: None,
            line_text: None,
        }
    }

    pub fn without_position(message: impl Into<String>) -> Self {
        Self::new(
            TokenSpan {
                token: Token::None,
                line: 0,
                column: 0,
            },
            message,
        )
    }
}

impl Warning {
    pub fn new(token: TokenSpan, message: impl Into<String>) -> Self {
        Self {
            token,
            message: message.into(),
            file: None,
            line_text: None,
        }
    }

    pub fn without_position(message: impl Into<String>) -> Self {
        Self::new(
            TokenSpan {
                token: Token::None,
                line: 0,
                column: 0,
            },
            message,
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = match self.kind {
            ErrorKind::Runtime => "Runtime Error",
            ErrorKind::Syntax => "Syntax Error",
        };

        write!(
            f,
            "[{}{}{}]: {}{}\n{}",
            COLOR_RED,
            kind,
            COLOR_RESET,
            STYLE_BOLD,
            self.message,
            COLOR_RESET,
        )?;

        if let Some(file) = &self.file {
            if self.token.line > 0 && self.token.column > 0 {
                write!(f, " {}:{}:{}", file, self.token.line, self.token.column)?;
            } else {
                write!(f, " {}", file)?;
            }
        }

        if let Some(line_text) = &self.line_text {
            if self.token.line > 0 && self.token.column > 0 {
                let width = std::cmp::max(4, self.token.line.to_string().len());
                let gutter = format!("{}{:>width$} |{}", COLOR_BLUE, "", COLOR_RESET, width = width);
                let prefix = format!("{}{:>width$} |{} ", COLOR_BLUE, self.token.line, COLOR_RESET, width = width);
                let marker_prefix = format!("{}{:>width$} |{} ", COLOR_BLUE, "", COLOR_RESET, width = width);
                let caret_len = std::cmp::max(1, self.token.display_len());
                let spaces = " ".repeat(self.token.column.saturating_sub(1));
                let carets = "^".repeat(caret_len);
                write!(
                    f,
                    "\n{}\n{}{}\n{}{}{}{}",
                    gutter,
                    prefix,
                    line_text,
                    marker_prefix,
                    spaces,
                    COLOR_RED,
                    carets
                )?;
                write!(f, "{}", COLOR_RESET)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}Warning]{}]: {}{}\n{}",
            COLOR_YELLOW,
            COLOR_RESET,
            STYLE_BOLD,
            self.message,
            COLOR_RESET,
        )?;

        if let Some(file) = &self.file {
            if self.token.line > 0 && self.token.column > 0 {
                write!(f, " {}:{}:{}", file, self.token.line, self.token.column)?;
            } else {
                write!(f, " {}", file)?;
            }
        }

        if let Some(line_text) = &self.line_text {
            if self.token.line > 0 && self.token.column > 0 {
                let width = std::cmp::max(4, self.token.line.to_string().len());
                let gutter = format!("{}{:>width$} |{}", COLOR_BLUE, "", COLOR_RESET, width = width);
                let prefix = format!("{}{:>width$} |{} ", COLOR_BLUE, self.token.line, COLOR_RESET, width = width);
                write!(f, "\n{}\n{}{}", gutter, prefix, line_text)?;
            }
        }

        Ok(())
    }
}