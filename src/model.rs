use serde::{Serialize, Deserialize};
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub gh_username: String,
    pub gh_org: String,
    pub gh_token: String,
    pub gh_repo: String,
    pub btrs_app_slug: String,
    pub btrs_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitriseAppResp {
    pub data: Vec<BitriseAppModel>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitriseAppModel {
    pub slug: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GHPullReponseModel {
    pub title: String,
    pub head: GHPullHeadAndBase,
    pub base: GHPullHeadAndBase
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GHPullHeadAndBase {
    #[serde(rename = "ref")]
    pub reference: String,
    pub sha: String,
    pub repo: GHPullRepo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GHPullRepo {
    pub ssh_url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitriseBuildParam {
   pub branch: String,
   pub branch_dest: String,
   pub pull_request_id: u16,
   pub commit_hash: String,
   pub commit_message: String,
   pub workflow_id: String,
   pub pull_request_merge_branch: String,
   pub pull_request_repository_url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitriseHookInfo {
    #[serde(rename = "type")]
    pub hook_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitriseRequestBody {
    pub hook_info: BitriseHookInfo,
    pub build_params: BitriseBuildParam,
}

#[derive(StructOpt)]
pub struct Commands {
    pub pattern: Pattern,
    pub prid: Option<u16>,
    pub env: Option<String>
}

pub enum Pattern { 
    Setup,
    Build
}

impl std::str::FromStr for Pattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "setup" => Ok(Pattern::Setup),
            "build" => Ok(Pattern::Build),
            _ => Err(format!("'{}' is not valid arguments", s)),
        }
    }
}
