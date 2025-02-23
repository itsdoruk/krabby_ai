use reqwest::Client as HttpClient;
use serde_json::json;
use std::error::Error;
use crate::models::message::{Message, ApiResponse};

pub struct Client {
    http_client: HttpClient,
    api_endpoint: String,
}

impl Client {
    pub fn new() -> Self {
        let config = crate::config::Config::new();
        let http_client = HttpClient::new();
        Client {
            http_client,
            api_endpoint: config.api_endpoint,
        }
    }

    pub async fn send_message(&self, messages: Vec<Message>) -> Result<String, Box<dyn Error>> {
        let body = json!({
            "messages": messages
        });

        let response = self.http_client
            .post(&self.api_endpoint)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let response_json: ApiResponse = response.json().await?;
        Ok(response_json.choices[0].message.content.clone())
    }
}