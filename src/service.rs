use crate::model;
use reqwest;
use reqwest::header::{AUTHORIZATION, ACCEPT, USER_AGENT};
use magic_crypt::MagicCryptTrait;
use std::fs::File;
use std::io::Write;

pub async fn get_gh_status(username: &str, token: &str) -> Result<bool, reqwest::Error> {
    let req_url = format!("https://api.github.com/orgs/kitabisa/members/{}", username);
    let client = reqwest::Client::new();
    let res = client.get(&req_url)
        .header(AUTHORIZATION, token)
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "ktbs-bitrise-cli/0.1")
        .send()
        .await?;
    match res.status() {
        reqwest::StatusCode::OK => { Ok(true) },
        reqwest::StatusCode::NO_CONTENT => { Ok(true) },
        _ => { Ok(false) }
    }
}

pub async fn get_bitrise_apps(token: &str) -> Result<Vec<model::BitriseAppModel>, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get("https://api.bitrise.io/v0.1/apps")
        .header(AUTHORIZATION, token)
        .header(ACCEPT, "application/json")
        .header(USER_AGENT, "ktbs-bitrise-cli/0.1")
        .send()
        .await?;
    let data: model::BitriseAppResp = res.json().await?;
    Ok(data.data) 
}

pub fn encrypt_config(model_config: &model::Config) {
    let mc = new_magic_crypt!(dotenv!("HASH_SECRET"), 256);    
    let json = serde_json::to_string(model_config).unwrap();
    let encrypted = mc.encrypt_str_to_base64(json);
    let mut file = File::create("Config").expect("Unable to create file");
    file.write_all(encrypted.as_bytes()).expect("Unable to write");
    println!("Success encrypt the config data");
}
