#![allow(dead_code)]

use anyhow::{Ok, Result};
use reqwest::header::HeaderMap;
use tracing::error;

use crate::domain::openai::{answers::Answers, question::Question};

#[derive(Debug)]
pub struct OpenAIChatClient {
    client: reqwest::Client,
    api_domain: String,
}

impl OpenAIChatClient {
    /// Creates a new [`OpenAIClient`].
    ///
    ///
    /// let open_client = OpenAIClient::new(
    ///    "Bearer {OpenAI token}".to_string(),
    ///    "https://api.openai.com".to_string(),
    /// );
    /// let ans = open_client
    ///    .query("What is Rust language?".to_string())
    ///    .await?;
    /// # Panics
    ///
    /// Panics if .
    pub fn new(api_key: String, api_domain: String) -> Self {
        let mut headermap = HeaderMap::new();
        headermap.insert("Authorization", api_key.parse().unwrap());
        headermap.insert("Content-Type", "application/json".parse().unwrap());
        let client = reqwest::Client::builder()
            .default_headers(headermap)
            .build()
            .unwrap();
        Self { client, api_domain }
    }

    pub async fn query(&self, question: String) -> Result<Answers> {
        let question =
            Question::new_with_default("gpt-3.5-turbo".to_string(), question, "bigduu".to_string());
        let endpoint = format!("{}/v1/chat/completions", self.api_domain);
        let response = self
            .client
            .post(endpoint.as_str())
            .json(&question)
            .send()
            .await
            .expect("Failed to send request");
        if !response.status().is_success() {
            error!("Failed to send request: {}", response.text().await.unwrap());
            return Ok(Answers::default());
        }
        let ans = response
            .json::<Answers>()
            .await
            .expect("Failed to parse response");
        Ok(ans)
    }
}
