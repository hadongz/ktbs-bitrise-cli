use std::io;
use std::io::Write;
use structopt::StructOpt;
use rpassword::*;
use serde::{Serialize, Deserialize};

enum Pattern { 
    Start,
}

impl std::str::FromStr for Pattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Pattern::Start),
            _ => Err(format!("'{}' is not valid arguments", s)),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Config {
    gh_username: String,
    gh_token: String,
}

#[derive(StructOpt)]
struct Commands {
    pattern: Pattern,
}

fn main() {
    let args = Commands::from_args();

    match args.pattern {
        Pattern::Start => { start() }
    }
}

fn start() {
    println!("====== Link to your Github ======");
    
    print_one_line("username: ");
    let gh_username = read_line();
    print_one_line("token: ");
    let gh_token = read_password().unwrap();
    
    let config = Config {
        gh_username: gh_username,
        gh_token: gh_token
    };

    let serialized = serde_json::to_string(&config).unwrap();
    println!("{}", serialized)
}

fn print_one_line(s: &str) {
    print!("{}", s);
    io::stdout().flush().expect("Falied to flush");
}

fn read_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
