use std::env;
use std::path::{Path, PathBuf};
use std::process;

use rustyline::DefaultEditor;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

use sk::SKInterpreter;
use sk::core::value::Value;

fn run(path: &Path) {
    let mut interpreter = SKInterpreter::new();

    match interpreter.execute(&path) {
        Ok(value) => {
            if value != Value::None {
                println!("{}", value);
            }
        }
        Err(e) => {
            log::error!("Runtime Error: {}", e);
            process::exit(1)
        }
    }
}

fn run_repl() {
    let mut interpreter = SKInterpreter::new();
    let mut rl = DefaultEditor::new().expect("Failed to create editor");
    
    println!("SK REPL ({}). Type 'exit' to quit.", VERSION);

    loop {
        let readline = rl.readline(">> ");
        
        match readline {
            Ok(line) => {
                let source = line.trim();
                
                if source == "exit" {
                    break;
                }
                
                if source.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(source);


                match interpreter.execute_string(source.to_string()) {
                    Ok(value) => {
                        if value != Value::None {
                            println!("{}", value);
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn check(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("Couldn't find file: '{}'", path.display()));
    }

    match path.extension().and_then(|e| e.to_str()) {
        Some("sk") => {}
        _ => log::warn!("Consider using the '.sk' extension!"),
    }

    Ok(())
}

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Warn)
        .format_timestamp(None)
        .init();

    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        run_repl();
        return;
    }

    if args.contains(&"--version".to_string()) {
        println!("{} v.{}", NAME, VERSION);
        process::exit(0);
    }

    if args.len() > 1 {
        help();
        process::exit(1);
    }

    let path = PathBuf::from(&args[0]);

    if let Err(e) = check(&path) {  // check file is valid
        log::error!("{}", e);
        process::exit(1)
    }

    run(&path);
}

fn help() {
    println!("{} - {}", NAME, VERSION);
    println!("usage: {} : starts a repl interpreter.", NAME);
    println!("       {} <filename> : runs the file at the given path.", NAME);
    println!("       {} --version : shows interpreter's version.", NAME);
}