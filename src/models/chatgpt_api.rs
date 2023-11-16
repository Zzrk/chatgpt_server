use std::env;

use reqwest::Error;
use rocket::serde::json::serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatBody {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatChoice {
    pub index: i32,
    pub message: ChatMessage,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: ChatUsage,
}

pub struct ChatGPTApi {
    pub base_url: String,
    pub api_key: String,
}

impl ChatGPTApi {
    pub async fn chat(&self, question: String) -> Result<String, Error> {
        let proxy = reqwest::Proxy::all(env::var("HTTP_PROXY").unwrap_or("".to_string())).unwrap();
        let client = reqwest::Client::builder().proxy(proxy).build()?;

        let url = format!("{}/chat/completions", self.base_url);
        let token = format!("Bearer {}", self.api_key);
        let body = ChatBody {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: question,
            }],
            temperature: 0.7,
        };

        let response = client
            .post(url)
            .header(reqwest::header::AUTHORIZATION, token)
            .json(&body)
            .send()
            .await?;

        let response = response.text().await?;
        let response = serde_json::from_str::<ChatResponse>(response.as_str()).expect("json error");

        Ok(response.choices[0].message.content.clone())
    }
}
