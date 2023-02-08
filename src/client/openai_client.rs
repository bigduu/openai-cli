#![allow(dead_code)]

use anyhow::{Ok, Result};
use reqwest::header::HeaderMap;

use crate::domain::openai::{answers::Answers, model::Model, question::Question};

#[derive(Debug)]
pub struct OpenAIClient {
    client: reqwest::Client,
    api_domain: String,
}

impl OpenAIClient {
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

    pub async fn get_models(&self) -> Result<Model> {
        let endpoint = format!("{}/v1/models", self.api_domain);
        let response = self.client.get(endpoint.as_str()).send().await?;
        let model: Model = response.json().await?;
        Ok(model)
    }

    pub async fn query(&self, question: String) -> Result<Answers> {
        let question = Question::new_with_default(
            "text-davinci-003".to_string(),
            question,
            "bigduu".to_string(),
        );
        let endpoint = format!("{}/v1/completions", self.api_domain);
        let response = self
            .client
            .post(endpoint.as_str())
            .json(&question)
            .send()
            .await
            .expect("Failed to send request");
        let ans = response
            .json::<Answers>()
            .await
            .expect("Failed to parse response");
        Ok(ans)
    }
}
