#![allow(dead_code)]
use anyhow::{Ok, Result};
use async_trait::async_trait;
use serde_json::json;
use std::time::Duration;
use uuid::Uuid;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, COOKIE, USER_AGENT};

use crate::{client::CompletionResponse, domain::chat_gpt::session_result::SessionResult};

use super::ChatGptClient;

#[derive(Debug, Clone)]
pub struct GPTClient {
    base_url: String,
    client: reqwest::Client,
    access_token: Option<String>,
    session_token: String,
}

#[async_trait]
impl ChatGptClient<String> for GPTClient {
    /// "https://chat.openai.com/backend-api".to_string()
    fn new(token: String, base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .default_headers(construct_headers(token.clone()))
            .timeout(Duration::from_secs(100))
            .build()
            .expect("Failed to build GPTClient");
        Self {
            base_url,
            client,
            access_token: None,
            session_token: token,
        }
    }

    async fn query<'a>(&'a mut self, message: String) -> Result<super::CompletionResponse<String>> {
        if self.access_token.is_none() {
            self.refresh_access_token().await?;
        }
        let access_token = self
            .access_token
            .as_ref()
            .expect("No access token provided");

        let uuidv4 = Uuid::new_v4();
        let conv_id = Uuid::new_v4();
        let params = json!(
            {
              "action": "next",
              "messages": [
                {
                  "id": uuidv4.to_string(),
                  "role": "user",
                  "content": {
                    "content_type": "text",
                    "parts": vec!(message),
                  }
                }
              ],
              "model": "text-davinci-002-render",
              "parent_message_id": conv_id.to_string(),

        });
        let res = self
            .client
            .post(format!("{}/{}", self.base_url, "conversation"))
            .json(&params)
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(format!("Bearer {access_token}").as_str()).unwrap(),
            )
            .send()
            .await?;

        Ok(CompletionResponse {
            response_status: res.status().as_u16(),
            result: res.text().await?,
        })
    }
}

impl GPTClient {
    pub async fn refresh_access_token(&mut self) -> Result<()> {
        let session_endpoint = "https://chat.openai.com/api/auth/session";
        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_str(
                format!(
                    "__Secure-next-auth.session-token={}",
                    self.session_token.clone()
                )
                .as_str(),
            )
            .unwrap(),
        );

        let res = self
            .client
            .get(session_endpoint)
            .headers(headers)
            .send()
            .await?
            .json::<SessionResult>()
            .await?;

        self.access_token = Some(res.access_token);
        Ok(())
    }
}

fn construct_headers(token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
            AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36",
        ),
    );
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {token}").as_str()).unwrap(),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    headers
}
