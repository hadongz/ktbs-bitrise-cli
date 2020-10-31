use serde::{Serialize, Deserialize};
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub gh_username: String,
    pub gh_token: String,
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

#[derive(StructOpt)]
pub struct Commands {
    pub pattern: Pattern,
}

pub enum Pattern { 
    Setup,
}

impl std::str::FromStr for Pattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "setup" => Ok(Pattern::Setup),
            _ => Err(format!("'{}' is not valid arguments", s)),
        }
    }
}
