pub mod build_csv_item;
pub mod get_study_hits_by_query;
pub mod load_config;
pub mod model;
pub mod write_to_csv;

use anyhow::anyhow;
use anyhow::Result;
use build_csv_item::*;
use get_study_hits_by_query::*;
use load_config::*;
use write_to_csv::*;

pub async fn run() -> Result<()> {
    // config
    let mut config = load_config().await?;
    println!("query: {:#?}", config);
    if config.query.is_empty() {
        return Err(anyhow!("query cannot be empty!"));
    }
    if config.keywords.is_empty() {
        return Err(anyhow!("keywords cannot be empty!"));
    }

    // config.keywords to_lowercase
    config
        .keywords
        .iter_mut()
        .for_each(|k| *k = k.to_lowercase());

    let client = reqwest::Client::builder()
        .user_agent("Chrome/96.0.4664.110")
        .build()
        .unwrap();

    println!("searching ...");
    let hits_set = get_study_hits_by_query(&client, &config.query).await?;

    let mut result = vec![];
    for hit in hits_set {
        if let Some(item) = build_csv_item(&client, &config, &hit).await {
            result.push(item)
        }
    }

    result.sort_by(|a, b| a.id.cmp(&b.id));
    println!("write to csv ...");
    write_to_csv(&result).await?;

    Ok(())
}
