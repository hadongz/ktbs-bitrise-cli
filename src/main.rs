#[macro_use] extern crate magic_crypt;
#[macro_use] extern crate dotenv_codegen;

use rpassword::{read_password};
use structopt::StructOpt;
use reqwest;

mod model;
mod common;
mod service;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = model::Commands::from_args();

    match args.pattern {
        model::Pattern::Setup => { setup().await? }
    }
    Ok(())
}

async fn setup() -> Result<(), reqwest::Error>  {
    println!("====== Setup the integration ======");
    
    common::print_one_line("github username: ");
    let gh_username = common::read_line();
    common::print_one_line("github personal accees token: ");
    let personal_gh_token = read_password().unwrap();
    let gh_token = common::convert_to_bas64(&gh_username, &personal_gh_token);
    let is_member = service::get_gh_status(&gh_username, &gh_token).await?;
    if is_member {
        println!("Success you are a member of Kitabisa");
        common::print_one_line("bitrise personal access token: ");
        let btrs_token = read_password().unwrap();
        let btrs_apps: Vec<model::BitriseAppModel> = service::get_bitrise_apps(&btrs_token).await?;
        println!("Choose app: ");
        for (i, item) in btrs_apps.iter().enumerate() {
            println!("{}. {}", i + 1, item.title)
        }
        let choosen_app = common::read_line();
        let app = btrs_apps
                    .into_iter()
                    .find(|item| item.title == choosen_app);
        match app {
            Some(item) => { 
                let config = model::Config{ 
                    gh_username: gh_username,
                    gh_token: gh_token,
                    btrs_app_slug: item.slug,
                    btrs_token: btrs_token,
                };
                service::encrypt_config(&config);
            }
            None => { println!("Choose the right app") }
        }
    } else {
        println!("Sorry you are not a member");
    }
    Ok(())
}
