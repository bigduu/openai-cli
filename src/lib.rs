use anyhow::Result;
use client::openai_client::OpenAIChatClient;
use config::token_info::OpenAIInfo;
use domain::openai::answers::Answers;
use lazy_static::lazy_static;

pub mod client;
pub mod config;
pub mod domain;

lazy_static! {
    static ref OPENAI_INFO: OpenAIInfo = OpenAIInfo::new("application.yml".to_string()).unwrap();
    static ref OPEN_CLIENT: OpenAIChatClient =
        OpenAIChatClient::new(OPENAI_INFO.get_token(), OPENAI_INFO.base_url());
}

pub async fn do_query(query: String) -> Result<Answers> {
    OPEN_CLIENT.query(query).await
}
