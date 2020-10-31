use std::io::Write;
use std::io;
use base64::{encode};

pub fn print_one_line(s: &str) {
    print!("{}", s);
    io::stdout().flush().expect("Falied to flush");
}

pub fn read_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

pub fn convert_to_bas64(username: &str, token: &str) -> String {
    let token = format!("{}:{}", username, token);
    let encoded = format!("Basic {}", encode(token));
    encoded.to_string()
}
