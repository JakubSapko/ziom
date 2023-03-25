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

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let command: &String = &args[1];

    // match command.as_str() {
    //     "help" => println!("This will print help"),
    //     "config" => println!("This will setup config"),
    //     _ => println!("This will print error message"),
    // }

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Config(api_key)) => {api::config::config::handle_config(&api_key.api_key)},
        Some(Commands::Generate(_)) => {api::caller::caller::read_api_key()},
        None => {},
    }
}
