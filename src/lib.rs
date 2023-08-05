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
    println!("config: {:#?}", config);
    if config.query.is_empty() {
        return Err(anyhow!("query cannot be empty!"));
    }
    if config.keywords.is_empty() {
        return Err(anyhow!("keywords cannot be empty!"));
    }

    let client = reqwest::Client::builder()
        .user_agent("Chrome/96.0.4664.110")
        .build()?;

    println!("searching ...");
    let hits_set = get_study_hits_by_query(&client, &config.query).await?;

    let mut tasks = Vec::with_capacity(hits_set.len());
    let tokio_task_slepp = config.tokio_task_sleep.unwrap_or(300);
    for hit in hits_set {
        let client = client.clone();
        let config = config.clone();
        let task = tokio::spawn(async move { build_csv_item(&client, &config, &hit).await });
        tasks.push(task);
        tokio::time::sleep(tokio::time::Duration::from_millis(tokio_task_slepp)).await;
    }

    let mut results = Vec::with_capacity(tasks.len());
    for task in tasks {
        match task.await {
            Ok(o) => {
                if let Some(csv_item) = o {
                    results.push(csv_item)
                }
            }
            Err(e) => println!("{:#?}", e),
        }
    }

    results.sort_by(|a, b| a.id.cmp(&b.id));
    println!("write to csv ...");
    write_to_csv(&results).await?;

    Ok(())
}
