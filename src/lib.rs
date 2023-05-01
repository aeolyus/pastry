use anyhow::Result;
use reqwest;
use reqwest::blocking::{multipart, Client};
use std::io::{self, Read};

// TODO: Make adjustable
const ENDPOINT: &str = "https://0x0.st";
/// Package the input string into a multipart form
fn upload(input: String) -> Result<String, reqwest::Error> {
    let mut data = multipart::Form::new();
    let part = multipart::Part::text(input).file_name("");
    data = data.part("file", part);

    // Send a POST request to the endpoint
    let client = Client::new();
    let response = client.post(ENDPOINT).multipart(data).send()?;

    return response.text();
}

/// Read the input from stdin
fn read_input() -> Result<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    return Ok(input);
}

/// Reads from stdin and uploads to a pastebin backend
pub fn pastry() -> Result<String> {
    let result = read_input()?;
    let url = upload(result)?;
    return Ok(url);
}
