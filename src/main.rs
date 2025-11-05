use anyhow::{Result, Error};
use clap::{Parser};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Write, stdin, stdout};
use std::process::exit;

#[derive(Parser)]
#[command(name = "misty-db")]
struct CliArgs{
    command: String,

    #[arg(short, long)]
    persistent: bool,

    #[arg(short, long, required_if_eq("persistent", "true"))]
    mount: Option<String>, 
}

impl Display for CliArgs{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Command: {}, Persistent: {:?}, Mount: {:?}",
        self.command, self.persistent, self.mount)
    }
}


fn main() {

    let args = CliArgs::parse();

    print!("-- Welcome to MistyDB --\n");

    if args.persistent {
        match &args.mount {
            Some(path) => print!("-- Mounting database at path: {}\n", path),
            None => exit(-1)
        }
    } else {
        print!("-- Running in in-memory mode.\n");
    }

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();

        stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

        input = input.trim().to_string();

        if input == "exit" || input == "quit"{
            println!("Goodbye!");
            break;
        }

        println!("Processing input...")
    }

}
