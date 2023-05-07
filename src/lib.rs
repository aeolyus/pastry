pub mod api;

use anyhow::Result;
use api::{EndpointApi, Pastebin, TheNullPointer};
use clap::Parser;
use std::io::{self, Read};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// The API we want to use for the endpoint
    #[arg(short, long)]
    api: EndpointApi,
}

/// Read the input from stdin
fn read_input() -> Result<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(input)
}

/// Reads from stdin and uploads to a pastebin backend
pub fn pastry() -> Result<String> {
    let result = read_input()?;
    let args = Args::parse();
    let endpoint_api: Box<dyn Pastebin> = match args.api {
        EndpointApi::TheNullPointer => Box::new(TheNullPointer {
            endpoint: "https://0x0.st".to_string(),
        }),
    };
    let url = endpoint_api.upload(result);
    url
}
