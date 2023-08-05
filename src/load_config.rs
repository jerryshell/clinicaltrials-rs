use anyhow::Result;

pub async fn load_config() -> Result<crate::model::config::Config> {
    let config_file = std::fs::File::open("config.json")?;
    let reader = std::io::BufReader::new(config_file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}
