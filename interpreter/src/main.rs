use std::env;
use std::path::{Path, PathBuf};
use std::process;
use log;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

use sk::SKInterpreter;

fn run(path: &Path) {
    let interpreter = SKInterpreter::new();

    match interpreter.execute(&path) {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(e) => {
            log::error!("Runtime Error: {}", e);
            process::exit(1)
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
        .init();

    let mut args = env::args().skip(1);

    let path = match args.next() {
        Some(p) => PathBuf::from(p),
        None => {
            help();
            process::exit(1)
        }
    };

    if args.next().is_some() {  // too many arguments
        help();
        process::exit(1)
    }

    if let Err(e) = check(&path) {  // check file is valid
        log::error!("{}", e);
        process::exit(1)
    }

    run(&path);
}

fn help() {
    println!("{} - {}", NAME, VERSION);
    println!("usage: {} <filename>", NAME);
}