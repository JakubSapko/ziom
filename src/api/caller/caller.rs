use confy::ConfyError;

use crate::api::{git_handler, config};

const PROMPT_BEG: &str = "{\"model\": \"text-davinci-003\",";

const PROMPT_END: &str = "\"temperature\": 0.7, \"max_tokens\": 256, \"top_p\": 1, \"frequency_penalty\": 0, \"presence_penalty\": 0 }";

const PROMPT_WITH_README: &str = "";

const PROMPT_WITHOUT_README: &str = "I will provide you a piece of code that is currently in a staged area of GIT repository. Please create a reasonable commit message describing the changes made by a code changes that are currently staged. Respond only with a commit message. Please follow the commit convention of: <type>[optional scope]: <description> where <type> means if some feature was added, deleted, fixed or moved, [optional scope] represents the code structure that was changed and <description> stands for a short description of changes that were made. Changes made: ";

pub async fn handle_commit() -> Result<String, Box<dyn std::error::Error>>   {
    let api_key = config::config::read_api_key()?;
    let msg = generate_commit_message(api_key);
    return msg.await;
}

pub async fn generate_commit_message(api_key: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let git_diff = git_handler::git_handler::get_staged_changes();
    let whole_prompt = format!("\"{}{}\"", PROMPT_WITHOUT_README, git_diff);
    let content = format!("{}\"prompt\": {}, {}", PROMPT_BEG, whole_prompt, PROMPT_END);
    // let content = format!("{}{}", PROMPT_BEG, PROMPT_END);
    println!("{}", content);
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
