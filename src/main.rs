use reqwest::blocking::{multipart, Client};
use std::io::{self, Read};

// TODO: Make adjustable
const ENDPOINT: &str = "https://0x0.st";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Package the input into a multipart form
    let mut data = multipart::Form::new();
    let part = multipart::Part::text(input).file_name("");
    data = data.part("file", part);

    // Send a POST request to the endpoint
    let client = Client::new();
    let response = client.post(ENDPOINT).multipart(data).send()?;

    // Print out the URL
    print!("{}", response.text()?);
    Ok(())
}
