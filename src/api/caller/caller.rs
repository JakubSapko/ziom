const PROMPT_BEG: &str = "{
                            \"model\": \"text-davinci-003\",
                        ";

const PROMPT_END: &str = "\temperature\": 0.7,
                          \"max_tokens\": 256,
                          \"top_p\": 1,
                          \"frequency_penalty\": 0,
                          \"presence_penalty\": 0
                          }
                        ";

const PROMPT_WITH_README: &str = "";

const PROMPT_WITHOUT_README: &str = "I'll provide you with a piece of code that is currently in a staged area of GIT repository. Please create a reasonable commit message describing the changes made by a code changes that are currently staged. Respond only with a commit message. Please follow the commit convention of: <type>[optional scope]: <description> where <type> means if some feature was added, deleted, fixed or moved, [optional scope] represents the code structure that was changed and <description> stands for a short description of changes that were made. Changes made:\n ";

pub async fn generate_commit_message(git_diff: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let content = format!("{}{}{}{}", PROMPT_BEG, PROMPT_WITHOUT_README, git_diff, PROMPT_END);

    let body = client
        .post("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .bearer_auth("")
        .body(content)
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}
