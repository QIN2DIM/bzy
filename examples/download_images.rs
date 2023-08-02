use anyhow::{Ok, Result};
use log::{debug, error, info};

use bzy_rs::project::Project;

async fn download_images(project: Project) -> Result<()> {
    debug!("{:#?}", project);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let project = Project::new();

    if !project.bzy_index.exists() {
        info!("正在下载任务队列 - save_path={:?}", project.bzy_index);
        project.pull_bzy_index().await?;
    };

    download_images(project).await.unwrap_or_else(|err| {
        error!("Opps~ {err}");
    });

    Ok(())
}
