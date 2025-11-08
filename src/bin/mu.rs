use clap::{Parser, Subcommand};
use misty_db::interpreter::Interpreter;
use std::fs;
use std::process::exit;

#[derive(Parser)]
#[command(name = "mu")]
#[command(about = "Mu language interpreter - scripting for MistyDB", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Mu script from a file
    Run {
        /// Path to the .mu source file
        path: String,
    },
    /// Start an interactive REPL
    Repl,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { path } => {
            run_file(&path);
        }
        Commands::Repl => {
            run_repl();
        }
    }
}

fn run_file(path: &str) {
    println!("-- Mu Interpreter --");
    println!("Running file: {}", path);
    
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            exit(1);
        }
    };

    match Interpreter::execute_full_pipeline(source) {
        Ok(_) => {
            println!("Execution completed successfully.");
        }
        Err(e) => {
            eprintln!("Error executing script: {}", e);
            exit(1);
        }
    }
}

fn run_repl() {
    use std::io::{stdin, stdout, Write};

    println!("-- Mu Interpreter REPL --");
    println!("Type 'exit', 'quit', or 'q' to exit.");

    loop {
        print!("mu> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_string();

        if input == "exit" || input == "quit" || input == "q" {
            println!("Goodbye!");
            break;
        }

        if input.is_empty() {
            continue;
        }

        match Interpreter::execute_full_pipeline(input) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
