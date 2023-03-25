use std::fs::File;
use std::io::prelude::*;
use crate::api::{git_handler, config};

const PROMPT_BEG: &str = "{\"model\": \"text-davinci-003\",";

const PROMPT_END: &str = "\"temperature\": 0.7, \"max_tokens\": 256, \"top_p\": 1, \"frequency_penalty\": 0, \"presence_penalty\": 0 }";

const PROMPT_WITH_README: &str = "A user is developing a software project, which is described by the project's README file as follows: ";

const GENERIC_PROMPT: &str = "I will now provide you a piece of code that is currently in a staged area of GIT repository. Please create a reasonable commit message describing the changes made by a code changes that are currently staged. Respond only with a commit message. Please follow the commit convention of: <type>[optional scope]: <description> where <type> means if some feature was added, deleted, fixed or moved, [optional scope] represents the code structure that was changed and <description> stands for a short description of changes that were made. Changes made: ";

pub async fn handle_commit(readme: &Option<String>) -> Result<String, Box<dyn std::error::Error>>   {
    let api_key = config::config::read_api_key()?;
    let msg = generate_commit_message(api_key, readme);
    return msg.await;
}

async fn generate_commit_message(api_key: String, readme: &Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut whole_prompt: String;

    match readme{
        Some(value) => whole_prompt = build_prompt_with_readme(value),
        None => whole_prompt = build_prompt_without_readme()
    }

    let content = format!("{}\"prompt\": {}, {}", PROMPT_BEG, whole_prompt, PROMPT_END);

    let body = client
        .post("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .bearer_auth(api_key)
        .body(content)
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}

fn build_prompt_without_readme() -> String {
    let git_diff = git_handler::git_handler::get_staged_changes().replace("\"", "").replace("\n", "").replace("\\", r"\\").replace("/", r"\/");
    let whole_prompt = format!("\"{}{}\"", GENERIC_PROMPT, git_diff);
    return whole_prompt;
}

fn build_prompt_with_readme(readme: &String) -> String {
    let git_diff = git_handler::git_handler::get_staged_changes().replace("\"", "").replace("\n", "").replace("\\", r"\\").replace("/", r"\/");
    let readme_content = read_readme_file(readme).ok().unwrap();
    let whole_prompt = format!("\"{}{}{}\"", readme_content, GENERIC_PROMPT, git_diff);
    return whole_prompt;
}

fn read_readme_file(readme: &String) -> Result<String, std::io::Error> {
    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    Ok(contents)
}