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
    let mut input = String::new();
    println!("You can input 'quit' to exit");
    println!("Please input your question:");
    loop {
        std::io::stdin().read_line(&mut input)?;
        if "quit" == input.trim() {
            break;
        }
        let ans = open_client.query(input.clone()).await?;
        let lines = ans.choices[0].text();
        lines.iter().for_each(|line| println!("{}", line));
        println!("----------------------------------------");
        println!("finish_reason:{},", ans.choices[0].finish_reason);
        println!("You can input continue or next question");
        println!("----------------------------------------");
        input.clear();
    }

    Ok(())
}
