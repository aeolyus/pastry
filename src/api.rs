pub mod gitlab;
pub mod thenullpointer;

use anyhow::Result;
use clap::ValueEnum;

pub trait Pastebin {
    fn upload(&self, input: String) -> Result<String>;
}

/// Represents the endpoint API type so we can interact properly
#[derive(Clone, ValueEnum, Debug)]
#[clap(rename_all = "lowercase")]
pub enum EndpointApi {
    TheNullPointer,
    GitLab,
}
