#[cfg(test)]
mod tests {

    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ACCEPT, CONTENT_TYPE};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    struct CompletionRequest {
        prompt: String,
        max_tokens: u32,
        temperature: f32,
        n: u32,
    }

    #[derive(Deserialize)]
    struct CompletionResponse {
        choices: Vec<CompletionChoice>,
    }

    #[derive(Deserialize)]
    struct CompletionChoice {
        text: String,
        index: u32,
        finish_reason: String,
        logprobs: Option<Logprobs>,
    }

    #[derive(Deserialize)]
    struct Logprobs {
        token_logprobs: Vec<f32>,
        top_logprobs: Vec<f32>,
        text_offset: Vec<u32>,
    }

    fn call_chat_gpt(prompt: &str) -> Result<String, reqwest::Error> {
        let endpoint = "https://api.openai.com/v1/completions";
        let api_key = "<YOUR_API_KEY_HERE>";

        let request_body = CompletionRequest {
            prompt: prompt.to_string(),
            max_tokens: 50,
            temperature: 0.7,
            n: 1,
        };

        let response = reqwest::Client::new()
            .post(endpoint)
            .headers(headers)
            .header(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap())
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json;charset=utf-8")
            .json(&request_body)
            .send()?
            .json::<CompletionResponse>()?;

        Ok(response.choices[0].text.to_string())
    }

    #[test]
    fn test() {
        println!(call_chat_gpt("你好").unwrap())
    }
}