use clap::Parser;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{stdin, stdout, Write};
use std::process::exit;

#[derive(Parser)]
#[command(name = "misty-db")]
#[command(about = "MistyDB - A simple database system", long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    persistent: bool,

    #[arg(short, long, required_if_eq("persistent", "true"))]
    mount: Option<String>,
}

impl Display for CliArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Persistent: {:?}, Mount: {:?}",
            self.persistent, self.mount
        )
    }
}

fn main() {
    let args = CliArgs::parse();

    println!("-- Welcome to MistyDB --");

    if args.persistent {
        match &args.mount {
            Some(path) => println!("-- Mounting database at path: {}", path),
            None => exit(-1),
        }
    } else {
        println!("Running in in-memory mode.");
    }

    println!("Type 'exit', 'quit', or 'q' to exit.");
    println!();

    loop {
        print!("misty> ");
        stdout().flush().unwrap();

        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");

        let input = input.trim().to_string();

        if input == "exit" || input == "quit" || input == "q" {
            println!("Goodbye!");
            break;
        }

        if input.is_empty() {
            continue;
        }

        println!("Processing: {}", input);
        // TODO: Add actual database command processing here
        // For now, this is just a placeholder REPL
    }
}
