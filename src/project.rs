use std::path::{Path, PathBuf};
use std::{fs, process};

use anyhow::{Ok, Result};
use env_logger;
use log::LevelFilter;
use reqwest::{Client, Proxy};

const HTTPS_PROXY: &str = "http://127.0.0.1:7890";

#[derive(Debug, Default)]
pub struct Project {
    pub img_dir: PathBuf,
    pub bzy_index: PathBuf,
}

impl Project {
    pub fn new() -> Self {
        let database = Path::new("database");
        let img_dir = database.join("backup");
        let bzy_index = database.join("BenZiYunMining.txt");

        fs::create_dir_all(&img_dir).unwrap_or_else(|err| {
            if !img_dir.exists() {
                println!("Could not create file database - {err}");
                process::exit(1)
            };
        });

        env_logger::builder().filter_level(LevelFilter::Info).init();

        Project { img_dir, bzy_index }
    }

    pub async fn pull_bzy_index(&self) -> Result<()> {
        let url = "https://github.com/QIN2DIM/bzy-rs/releases/download/bzy-db/BenZiYunMining.txt";

        let client = Client::builder()
            .proxy(Proxy::all(HTTPS_PROXY)?)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.188")
            .build()?;

        let bytes_db = client.get(url).send().await?.bytes().await?;

        fs::write(&self.bzy_index, bytes_db)?;

        Ok(())
    }
}
