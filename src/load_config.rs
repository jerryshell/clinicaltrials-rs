use anyhow::Result;

pub async fn load_config() -> Result<crate::model::config::Config> {
    let config_file = std::fs::File::open("config.json")?;
    let reader = std::io::BufReader::new(config_file);
    let mut config: crate::model::config::Config = serde_json::from_reader(reader)?;

    // keywords to_lowercase
    config
        .keywords
        .iter_mut()
        .for_each(|k| *k = k.to_lowercase());

    Ok(config)
}
