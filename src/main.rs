use anyhow::Result;
use config::token_info::OpenAIInfo;

use crate::client::openai_client::OpenAIClient;

mod client;
mod config;
mod domain;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let openai_info = OpenAIInfo::new("application.yml".to_string())?;
    let open_client = OpenAIClient::new(openai_info.get_token(), openai_info.base_url());
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please input your question");
        return Ok(());
    }
    args.remove(0);
    let query = args.join(" ");
    println!("Query question:\n [{}] \nplease wait...", query.clone());
    let ans = open_client.query(query).await?;
    let lines = ans.choices[0].text();
    lines.iter().for_each(|line| println!("{}", line));
    println!("----------------------------------------");
    println!("finish_reason:{},", ans.choices[0].finish_reason);
    println!("----------------------------------------");

    Ok(())
}
