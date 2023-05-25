use crate::Pastebin;
use anyhow::Result;
use clap::ValueEnum;
use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct GitLab {
    // The URL of the endpoint
    pub endpoint: String,
    pub token: String,
    pub visibility: Visibility,
}

#[derive(Debug, Serialize)]
struct SnippetFile {
    file_path: String,
    content: String,
}

#[derive(Debug, ValueEnum, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Private,
    Internal,
    Public,
}

#[derive(Debug, Serialize)]
struct NewSnippetRequest {
    title: String,
    visibility: Visibility,
    files: Vec<SnippetFile>,
}

impl Pastebin for GitLab {
    fn upload(&self, input: String) -> Result<String> {
        let snippet_file = SnippetFile {
            file_path: "pastry".to_string(),
            content: input,
        };
        let request_body = NewSnippetRequest {
            title: "pastry".to_string(),
            visibility: self.visibility,
            files: vec![snippet_file],
        };
        let client = Client::new();
        let resp = client
            .post(self.endpoint.to_string())
            .header("PRIVATE-TOKEN", self.token.to_string())
            .json(&request_body)
            .send()?;
        let resp_json = resp.json::<serde_json::Value>()?;
        let url = resp_json["web_url"].as_str().unwrap();
        Ok(url.to_string())
    }
}
