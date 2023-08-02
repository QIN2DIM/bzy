use std::fs;
use std::str::FromStr;

use anyhow::{Ok, Result};
use reqwest::{Client, Proxy, Url};

async fn get_text_db() -> Result<()> {
    let url = Url::from_str("https://github.com/QIN2DIM/bzy-rs/releases/download/bzy-db/BenZiYunMining.txt")?;
    let path = "BenZiYunMining.txt";

    let client = Client::builder()
        .proxy(Proxy::all("http://127.0.0.1:7890")?)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.188")
        .build()?;

    let bytes_db = client.get(url)
        .send().await?
        .bytes().await?;

    fs::write(path, bytes_db)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    get_text_db().await.unwrap_or_else(|err| {
        println!("Opps~ {err}")
    });

    Ok(())
}