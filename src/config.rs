use anyhow::Result;
use serde::{Deserialize, Serialize};

const CFG_FILE: &str = "config.yml";

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub node_url: String,
    pub addresses: Vec<String>,
}

pub async fn read_cfg() -> Result<Config, anyhow::Error> {
    let tokio_file = tokio::fs::File::open(CFG_FILE).await?.into_std().await;

    Ok(serde_yaml::from_reader(tokio_file)?)
}
