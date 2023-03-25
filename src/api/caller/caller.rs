
pub async fn generate_commit_message() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let content = std::fs::read_to_string("src/api/caller/sample.json").unwrap();
    let body = client
            .post("https://api.openai.com/v1/completions")
            .header("Content-Type", "application/json")
            .bearer_auth("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")
            // .body("\"model\": \"text-davinci-003\",
            // \"prompt\": \"Say hello to me\",
            // \"temperature\": \"0.7\",
            // \"max_tokens\": \"256\",
            // \"top_p\": \"1\",
            // \"frequency_penalty\": \"0\",
            // \"presence_penalty\": \"0\"")
            .body(content)
            .send()
            .await?
            .text()
            .await?;
    Ok(body)
}
