use crate::model;
use std::io::Write;
use std::io;
use base64::{encode};
use magic_crypt::MagicCryptTrait;
use std::fs;

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

pub fn get_config() -> model::Config {
    println!("====== Decrypting config file ======");
    let mc = new_magic_crypt!(dotenv!("HASH_SECRET"), 256);    
    let home_path = dirs::home_dir().unwrap().join(".trggr_config");
    let config_data = fs::read_to_string(home_path).expect("No config file, try setup first");
    let decrypt_config = mc.decrypt_base64_to_string(&config_data).expect("Failed to decrypt");
    let model: model::Config = serde_json::from_str(&decrypt_config).expect("Failed to map");
    model
}
