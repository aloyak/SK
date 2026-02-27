use std::process;
use std::borrow::Cow;
use rustyline::highlight::{Highlighter, CmdKind};
use rustyline::{Completer, Helper, Hinter, Validator};
use sk_lang::core::value::Value;
use sk_lang::SKInterpreter;

#[derive(Helper, Completer, Hinter, Validator)]
struct RLHelper;

const COLOR_KEYWORD: &str = "\x1b[35m";
const COLOR_LITERAL: &str = "\x1b[94m"; 
const COLOR_NUMBER: &str = "\x1b[36m";
const COLOR_STRING: &str = "\x1b[32m";
const COLOR_RESET: &str = "\x1b[0m";

impl Highlighter for RLHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        let mut output = String::new();
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '"' || c == '\'' {
                output.push_str(COLOR_STRING);
                output.push(c);
                while let Some(&_next) = chars.peek() {
                    let next_char = chars.next().unwrap();
                    output.push(next_char);
                    if next_char == c { break; }
                }
                output.push_str(COLOR_RESET);
            } else if c.is_ascii_digit() {
                output.push_str(COLOR_NUMBER);
                output.push(c);
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() || next == '.' {
                        if next == '.' {
                            let mut lookahead = chars.clone();
                            lookahead.next();
                            if lookahead.peek() == Some(&'.') { break; }
                        }
                        output.push(chars.next().unwrap());
                    } else { break; }
                }
                output.push_str(COLOR_RESET);
            } else if c.is_alphabetic() || c == '_' {
                let mut word = String::from(c);
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' || next == '!' {
                        word.push(chars.next().unwrap());
                    } else { break; }
                }
                let color = match word.as_str() {
                    "let" | "fn" | "if" | "else" | "elif" | "match" | "loop" | "for" | "in" | "import" | "as" | "pub" | "symbolic" | "quiet" | "panic!" | "panic" | "try" | "catch" | "strict" | "merge" | "unknown" | "any" => COLOR_KEYWORD,
                    "true" | "false" | "none" | "partial" => COLOR_LITERAL,
                    _ => COLOR_RESET,
                };
                output.push_str(color);
                output.push_str(&word);
                output.push_str(COLOR_RESET);
            } else if c == '.' && chars.peek() == Some(&'.') {
                output.push_str(COLOR_KEYWORD);
                output.push('.');
                output.push(chars.next().unwrap());
                output.push_str(COLOR_RESET);
            } else {
                output.push(c);
            }
        }
        Cow::Owned(output)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _kind: CmdKind) -> bool {
        true
    }
}

pub fn run_repl(safe_mode: bool, name: &str, version: &str) {
    let mut interpreter = SKInterpreter::new_with_options(safe_mode);
    let mut rl = rustyline::Editor::<RLHelper, rustyline::history::DefaultHistory>::new()
        .expect("Failed to create editor");
    
    rl.set_helper(Some(RLHelper));
    
    let mut buffer = String::new();

    println!(
        "{} REPL ({})\n  - Type 'exit!' to quit\n  - 'clear!' to clear the screen\n  - 'reset!' to clear the buffer.",
        name, version
    );

    loop {
        let prompt = if buffer.is_empty() { ">> " } else { ".. " };
        let readline = rl.readline(prompt);

        match readline {
            Ok(line) => {
                let source = line.trim();

                if source == "exit!" {
                    break;
                } else if source == "clear!" {
                    #[cfg(windows)] // Why does windows allways have to be different :(
                    {
                        let _ = process::Command::new("cmd")
                            .arg("/C")
                            .arg("cls")
                            .status();
                    }
                    #[cfg(not(windows))]
                    {
                        let _ = process::Command::new("clear").status();
                    }
                    buffer.clear();
                    continue;
                } else if source == "reset!" {
                    buffer.clear();
                    continue;
                }

                if source.is_empty() {
                    continue;
                }

                if !buffer.is_empty() {
                    buffer.push('\n');
                }
                buffer.push_str(&line);

                if !is_buffer_complete(&buffer) {
                    continue;
                }

                let _ = rl.add_history_entry(buffer.as_str());

                match interpreter.execute_string(buffer.clone()) {
                    Ok(value) => {
                        if value != Value::None {
                            println!("{}", value);
                        }
                        for warning in interpreter.take_warnings() {
                            eprintln!("{}", warning);
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                }

                buffer.clear();
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn is_buffer_complete(source: &str) -> bool {
    let mut brace_count = 0i32;
    let mut paren_count = 0i32;
    let mut bracket_count = 0i32;
    let mut in_string: Option<char> = None;
    let mut escaped = false;

    for ch in source.chars() {
        if let Some(quote) = in_string {
            if escaped {
                escaped = false;
                continue;
            }

            if ch == '\\' {
                escaped = true;
                continue;
            }

            if ch == quote {
                in_string = None;
            }

            continue;
        }

        match ch {
            '"' | '\'' => in_string = Some(ch),
            '{' => brace_count += 1,
            '}' => brace_count -= 1,
            '(' => paren_count += 1,
            ')' => paren_count -= 1,
            '[' => bracket_count += 1,
            ']' => bracket_count -= 1,
            _ => {}
        }
    }

    brace_count == 0 && paren_count == 0 && bracket_count == 0 && in_string.is_none()
}
