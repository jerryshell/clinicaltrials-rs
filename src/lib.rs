pub mod build_csv_item;
pub mod get_study_hits_by_query;
pub mod load_config;
pub mod model;
pub mod write_to_csv;

use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Result;
use build_csv_item::*;
use get_study_hits_by_query::*;
use load_config::*;
use write_to_csv::*;

pub async fn run() -> Result<()> {
    let config = Arc::new(load_config().await?);
    tracing::info!("config: {:#?}", config);
    if config.query.is_empty() {
        return Err(anyhow!("query cannot be empty!"));
    }
    if config.keywords.is_empty() {
        return Err(anyhow!("keywords cannot be empty!"));
    }

    let client = reqwest::Client::builder()
        .user_agent("Chrome/96.0.4664.110")
        .build()?;

    tracing::info!("searching ...");
    let hits_set = get_study_hits_by_query(&client, &config.query).await?;
    let hits_set_len = hits_set.len();
    tracing::info!("hits_set_len: {}", hits_set_len);

    let mut tasks = Vec::with_capacity(hits_set_len);
    let tokio_task_sleep = config.tokio_task_sleep.unwrap_or(200);
    let mut task_spawn_count = 0;
    for hit in hits_set {
        let client = client.clone();
        let config = config.clone();
        let task = tokio::spawn(async move { build_csv_item(&client, &config, &hit).await });
        tasks.push(task);
        task_spawn_count += 1;
        tracing::info!(
            "progress: {}/{} {}%",
            task_spawn_count,
            hits_set_len,
            (task_spawn_count as f32 / hits_set_len as f32) * 100.0f32
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(tokio_task_sleep)).await;
    }

    let mut results = Vec::with_capacity(hits_set_len);
    let mut error_count = 0;
    for task in tasks {
        match task.await {
            Ok(o) => {
                if let Some(csv_item) = o {
                    results.push(csv_item)
                }
            }
            Err(e) => {
                error_count += 1;
                tracing::error!("{:#?}", e);
            }
        }
    }
    tracing::info!("error count: {}", error_count);

    results.sort_by(|a, b| a.id.cmp(&b.id));
    tracing::info!("write to csv ...");
    write_to_csv(&results).await?;

    Ok(())
}
