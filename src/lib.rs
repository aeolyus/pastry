pub mod api;

use anyhow::Result;
use api::gitlab::{GitLab, Visibility};
use api::thenullpointer::TheNullPointer;
use api::{EndpointApi, Pastebin};
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use std::io::{self, Read};

#[derive(Parser)]
#[command(author, version, about, args_conflicts_with_subcommands = true)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    /// The API we want to use for the endpoint
    #[arg(short, long, default_value = "thenullpointer")]
    api: EndpointApi,
    /// Personal access token for API
    #[arg(short, long)]
    token: Option<String>,
    /// API URL
    #[arg(short, long)]
    url: Option<String>,
    /// Visibility
    #[arg(short, long, default_value = "public")]
    visibility: Visibility,
}

#[derive(Subcommand)]
enum Commands {
    /// Provides shell completion
    Completion {
        /// Shell to generate the completion for
        #[arg(short, long)]
        shell: Shell,
    },
}

/// Read the input from stdin
fn read_input() -> Result<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(input)
}

#[cold]
fn print_completion<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout())
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Completion { shell }) => {
            print_completion(*shell, &mut Args::command());
            return Ok(());
        }
        None => {}
    }

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
            visibility: args.visibility,
        }),
    };
    let url = endpoint_api.upload(result);
    match url {
        Ok(url) => {
            print!("{}", url);
            Ok(())
        }
        Err(err) => Err(err),
    }
}
