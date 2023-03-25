use clap::{Parser, Subcommand, Args};
mod api;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "ziom - a OpenAI-powered CLI for easy generating adequate messages for your commits!")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    // adds user's openAI key to config
    Config(Config),
    // checks currently staged files and generates commit message for them
    Generate(Generate),
}

#[derive(Args)]
struct Config {
    // user's openAI key
    api_key: Option<String>,
}

#[derive(Args)]
struct Generate {
    #[arg(short = 'c', long = "autocommit")]
    auto_commit: bool,
}
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Config(api_key)) => {api::config::config::handle_config(&api_key.api_key)},
        Some(Commands::Generate(_)) => {let msg = api::caller::caller::generate_commit_message().await;
            match msg {
                Ok(value) => println!("Msg: {}", value),
                Err(e) => println!("Ups {}", e)
            }
        },
        None => {},
    }
}
