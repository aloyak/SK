use std::io;
use std::env;
use std::path::{Path, PathBuf};
use std::process;

use rustyline::DefaultEditor;

const NAME: &str = env!("CARGO_BIN_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

use sk_lang::SKInterpreter;
use sk_lang::core::value::Value;

fn run(path: &Path) {
    let mut interpreter = SKInterpreter::new();

    match interpreter.execute(&path) {
        Ok(value) => {
            if value != Value::None {
                println!("{}", value);
            }
            for warning in interpreter.take_warnings() {
                eprintln!("{}", warning);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }
}

fn run_repl() {
    let mut interpreter = SKInterpreter::new();
    let mut rl = DefaultEditor::new().expect("Failed to create editor");
    
    println!("{} REPL ({}). Type 'exit' to quit.", NAME, VERSION);

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
                        for warning in interpreter.take_warnings() {
                            eprintln!("{}", warning);
                        }
                    }
                    Err(e) => eprintln!("{}", e),
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
        _ => eprintln!("Warning: Consider using the '.sk' extension!"),
    }

    Ok(())
}

fn create_proj(name: String) {
    let mut path = PathBuf::from(&name);
    if path.exists() {
        eprintln!("Error: A file or directory with the name '{}' already exists!", name);
        process::exit(1);
    }

    std::fs::create_dir(&path).expect("Failed to create project directory");
    path.push("main.sk");

    std::fs::write(&path, 
        &format!("// SK Version: {}\n\nprint('Hello, World!')", VERSION)
    ).expect("Failed to create main.sk");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        run_repl();
        return;
    }

    if args.contains(&"--version".to_string()) {
        println!("{} v{}", NAME, VERSION);
        process::exit(0);
    }

    if args.contains(&"--help".to_string()) {
        help();
        process::exit(0);
    }

    if args.len() > 2 {
        help();
        process::exit(1);
    }
    
    let mut path = PathBuf::from(&args[0]);

    if args.contains(&"--project".to_string()) {
        if &args[1] == "new" {
            
            let mut name = String::new();
            println!("New Project's Name:");
            io::stdin().read_line(&mut name).expect("Failed to read line");
            let name = name.trim().to_string();

            create_proj(name);

            process::exit(0);
        }

        path = PathBuf::from(&args[1]);
        path.push("main.sk");
    }
    
    if let Err(e) = check(&path) {  // check file is valid
        eprintln!("Error: {}", e);
        process::exit(1)
    }

    run(&path);
}

fn help() {
    println!("{} - {}", NAME, VERSION);
    println!("usage: {} : starts a repl interpreter.", NAME);
    println!("       {} <filename> : runs the file at the given path.", NAME);
    println!("       {} --project <path> : runs 'main.sk' at the given path.", NAME);
    println!("       {} --project new : creates a new project.", NAME);
    println!("       {} --version : shows interpreter's version.", NAME);
    println!("       {} --help : shows this dialog.", NAME);
}