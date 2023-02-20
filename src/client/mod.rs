#![allow(dead_code)]
pub mod gpt_client;
pub mod openai_client;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ChatGptClient<T> {
    fn new(token: String, base_url: String) -> Self;
    async fn query(&mut self, message: String) -> Result<CompletionResponse<T>>;
}

pub struct CompletionResponse<T> {
    response_status: u16,
    result: T,
}
