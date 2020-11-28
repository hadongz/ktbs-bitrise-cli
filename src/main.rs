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
        model::Pattern::Build => { build(&args.prid.unwrap(), &args.env.unwrap()).await? }
    }
    Ok(())
}

async fn setup() -> Result<(), reqwest::Error>  {
    println!("====== Setup the CLI ======");
    
    common::print_one_line("github username: ");
    let gh_username = common::read_line();
    common::print_one_line("your orgnatization: ");
    let gh_org = common::read_line();
    common::print_one_line("github personal accees token: ");
    let personal_gh_token = read_password().unwrap();
    let gh_token = common::convert_to_bas64(&gh_username, &personal_gh_token);
    let is_member = service::get_gh_status(&gh_username, &gh_org, &gh_token).await?;
    if is_member {
        println!("Success, you are a member of organization");
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
                    gh_org: gh_org,
                    gh_token: gh_token,
                    gh_repo: item.title,
                    btrs_app_slug: item.slug,
                    btrs_token: btrs_token,
                };
                service::encrypt_config(&config);
                println!("====== Setup successful ======");
            }
            None => { println!("Choose the right app") }
        }
    } else {
        println!("Sorry, you are not a member of the organization");
    }
    Ok(())
}

async fn build(id: &u16, env: &str) -> Result<(), reqwest::Error> {
    println!("{} {}", id, env);
    let config = common::get_config();
    let gh_info: model::GHPullReponseModel = service::get_pull_info(&id, &config).await?;
    let bitrise_hook_info = model::BitriseHookInfo { 
        hook_type: "bitrise".into(),
    };
    let bitrise_build_param = model::BitriseBuildParam {
        branch: gh_info.head.reference,
        branch_dest: gh_info.base.reference,
        pull_request_id: id.clone(),
        commit_hash: gh_info.head.sha,
        commit_message: gh_info.title,
        workflow_id: env.to_string(),
        pull_request_merge_branch: format!("pull/{}/merge", &id),
        pull_request_repository_url: gh_info.head.repo.ssh_url
    };
    let bitrise_req_body = model::BitriseRequestBody {
        hook_info: bitrise_hook_info,
        build_params: bitrise_build_param,
    };
    let json_string = serde_json::to_string(&bitrise_req_body).unwrap();
    let is_success = service::post_build(json_string, &config.btrs_app_slug, &config.btrs_token).await?;
    if is_success {
        println!("====== Success trigger build ======");
    } else {
        println!("====== Failed trigger build ======");
    }
    Ok(())
}
