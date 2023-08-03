use std::fs;
use std::path::PathBuf;

use anyhow::{Ok, Result};
use chrono::prelude::*;
use log::{error, info};
use rand::seq::SliceRandom;
use reqwest::Client;
use tokio::time::Instant;

use bzy_rs::project::Project;

const BATCH_SIZE: usize = 10;

async fn run_collector(project: &Project) -> Result<()> {
    let mut urls = project.load_bzy_index()?;
    info!("读入 {} 条待处理链接", urls.len());

    info!("正在下载随机样本 - batch={BATCH_SIZE}");

    // 创建样本缓存目录
    let dt = Local::now().format("%Y-%m-%d %H%M%S").to_string();
    let save_path = project.img_dir.join(dt);
    if !save_path.exists() {
        fs::create_dir_all(&save_path)?;
    }

    // 打乱任务队列
    urls.shuffle(&mut rand::thread_rng());

    // 初始化协程池
    let client = Client::new();
    let mut handles = Vec::new();

    // 启动任务
    for i in 1..BATCH_SIZE + 1 {
        let url = &urls[i];
        let handle = tokio::spawn(download_images(
            client.clone(),
            url.clone(),
            save_path.clone(),
        ));
        handles.push(handle)
    }
    for handle in handles {
        handle.await??;
    }

    Ok(())
}

async fn download_images(client: Client, url: String, save_path: PathBuf) -> Result<()> {
    let img = client.get(&url).send().await?.bytes().await?;
    let filename = url.split("/").last().unwrap().to_string();
    let p = save_path.join(&filename);
    fs::write(p, img)?;
    info!("下载成功 - filename={}", &filename);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let project = Project::new();

    if !project.bzy_index.exists() {
        info!("正在下载任务队列 - save_path={:?}", project.bzy_index);
        project.pull_bzy_index().await?;
    };

    let start = Instant::now();
    run_collector(&project)
        .await
        .unwrap_or_else(|err| error!("Opps~ {err}"));
    info!("任务结束 - Time elapsed: {:?}", start.elapsed());
    Ok(())
}
