pub mod api;

use anyhow::Result;
use api::{EndpointApi, GitLab, Pastebin, TheNullPointer, Visibility};
use clap::Parser;
use std::io::{self, Read};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// The API we want to use for the endpoint
    #[arg(short, long)]
    api: EndpointApi,
    /// Personal access token for API
    #[arg(short, long)]
    token: Option<String>,
    /// API URL
    #[arg(short, long)]
    url: Option<String>,
    /// Visibility
    #[arg(short, long)]
    visibility: Option<Visibility>,
}

/// Read the input from stdin
fn read_input() -> Result<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(input)
}

/// Reads from stdin and uploads to a pastebin backend
pub fn pastry() -> Result<String> {
    let args = Args::parse();
    let result = read_input()?;
    let endpoint_api: Box<dyn Pastebin> = match args.api {
        EndpointApi::TheNullPointer => Box::new(TheNullPointer {
            endpoint: args.url.unwrap_or("https://0x0.st".to_string()),
        }),
        EndpointApi::GitLab => Box::new(GitLab {
            endpoint: args
                .url
                .unwrap_or("https://gitlab.com/api/v4/snippets".to_string()),
            token: args.token.unwrap_or("".to_string()),
            visibility: args.visibility.unwrap_or(Visibility::Public),
        }),
    };
    let url = endpoint_api.upload(result);
    url
}
