use anyhow::Result;
use clap::{Parser, ValueEnum};
use reqwest::blocking::{multipart, Client};
use std::io::{self, Read};

trait Pastebin<'a> {
    fn upload(self, input: String) -> Result<String>;
}

#[derive(Clone, Debug)]
struct TheNullPointer<'a> {
    // The URL of the endpoint
    endpoint: &'a str,
}

impl<'a> Pastebin<'a> for TheNullPointer<'a> {
    fn upload(self, input: String) -> Result<String> {
        // Package the input string into a multipart form
        let mut data = multipart::Form::new();
        let part = multipart::Part::text(input).file_name("");
        data = data.part("file", part);
        // Send a POST request to the endpoint
        let client = Client::new();
        let response = client.post(self.endpoint).multipart(data).send()?;

        response.text().map_err(anyhow::Error::from)
    }
}

/// Represents the endpoint API type so we can interact properly
#[derive(Clone, ValueEnum, Debug)]
enum EndpointApi {
    TheNullPointer,
}

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
    let url = match args.api {
        EndpointApi::TheNullPointer => TheNullPointer {
            endpoint: "https://0x0.st",
        }
        .upload(result),
    }?;
    Ok(url)
}
