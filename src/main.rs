use anyhow::Result;
use config::token_info::OpenAIInfo;

use crate::client::openai_client::OpenAIChatClient;

mod client;
mod config;
mod domain;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let openai_info = OpenAIInfo::new("application.yml".to_string())?;
    let open_client = OpenAIChatClient::new(openai_info.get_token(), openai_info.base_url());
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please input your question");
        return Ok(());
    }
    args.remove(0);
    let query = args.join(" ");
    println!("Query question:\n [{}] \nplease wait...", query.clone());
    do_query(&open_client, query).await?;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input == "q" {
            break;
        }
        do_query(&open_client, input).await?;
    }

    Ok(())
}

async fn do_query(client: &OpenAIChatClient, query: String) -> anyhow::Result<()> {
    let ans = client.query(query).await?;
    ans.choices
        .iter()
        .map(|choice| {
            let mut message = choice.message.clone();
            message.content = message
                .content
                .replace('\u{200b}', "")
                .trim()
                .trim_matches('\n')
                .to_string();
            (message, choice.finish_reason.clone())
        })
        .filter(|choice| !choice.0.content.is_empty())
        .for_each(|choice| {
            println!("----------------------------------------");
            println!("{}", choice.0.content);
            println!("----------------------------------------");
            println!("finish_reason:{},", choice.1);
            println!("----------------------------------------");
        });
    Ok(())
}
