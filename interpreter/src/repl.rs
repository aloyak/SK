use std::process;

use rustyline::DefaultEditor;

use sk_lang::core::value::Value;
use sk_lang::SKInterpreter;

pub fn run_repl(safe_mode: bool, name: &str, version: &str) {
    let mut interpreter = SKInterpreter::new_with_options(safe_mode);
    let mut rl = DefaultEditor::new().expect("Failed to create editor");
    let mut buffer = String::new();

    println!(
        "{} REPL ({}).\n - Type 'exit!' to quit\n - 'clear!' to clear the screen\n - 'reset!' to clear the buffer.",
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
