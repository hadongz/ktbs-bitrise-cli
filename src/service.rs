use crate::model;
use reqwest;
use dirs;
use reqwest::header::{AUTHORIZATION, ACCEPT, USER_AGENT};
use magic_crypt::MagicCryptTrait;

pub async fn get_gh_status(username: &str, token: &str) -> Result<bool, reqwest::Error> {
    println!("====== Checking Kitabisa membership ======");
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
    println!("====== Checking the apps on your bitrise account ======");
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
    println!("====== Encrypt config data ======");
    let mc = new_magic_crypt!(dotenv!("HASH_SECRET"), 256);    
    let json = serde_json::to_string(model_config).unwrap();
    let encrypted = mc.encrypt_str_to_base64(json);
    let home_path = dirs::home_dir().expect("Error").join(".ktbs_config");
    std::fs::write(home_path, encrypted.as_bytes()).expect("Unable to write");
}

pub async fn get_pull_info(id: &u16, config: &model::Config) -> Result<model::GHPullReponseModel, reqwest::Error> {
    println!("====== Getting pull repo information ======");
    let req_url = format!("https://api.github.com/repos/kitabisa/{}/pulls/{}", config.gh_repo, id);
    let client = reqwest::Client::new();
    let res = client.get(&req_url)
        .header(AUTHORIZATION, &config.gh_token)
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "ktbs-bitrise-cli/0.1")
        .send()
        .await?;
    let result: model::GHPullReponseModel = res.json().await?;
    Ok(result)
}

pub async fn post_build(param: String, app_slug: &str, token: &str) -> Result<bool, reqwest::Error> {
    println!("====== Requesting build to bitrise ======");
    let req_url = format!("https://api.bitrise.io/v0.1/apps/{}/builds", app_slug);
    let client = reqwest::Client::new();
    let res = client.post(&req_url)
        .header(AUTHORIZATION, token)
        .header(ACCEPT, "application/json")
        .header("Content-Type", "application/json")
        .header(USER_AGENT, "ktbs-bitrise-cli/0.1")
        .body(param)
        .send()
        .await?;
    match res.status() {
        reqwest::StatusCode::OK => { Ok(true) },
        _ => { Ok(false) }
   }
}
