use crate::Pastebin;
use anyhow::Result;
use reqwest::blocking::{multipart, Client};

#[derive(Clone, Debug)]
pub struct TheNullPointer {
    // The URL of the endpoint
    pub endpoint: String,
}

impl Pastebin for TheNullPointer {
    fn upload(&self, input: String) -> Result<String> {
        // Package the input string into a multipart form
        let mut data = multipart::Form::new();
        let part = multipart::Part::text(input).file_name("");
        data = data.part("file", part);
        // Send a POST request to the endpoint
        let client = Client::new();
        let response = client
            .post(self.endpoint.to_string())
            .multipart(data)
            .send()?;
        response.text().map_err(anyhow::Error::from)
    }
}
