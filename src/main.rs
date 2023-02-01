use anyhow::Result;

use coin_watcher::config::*;
use iota_client::{
    Client,
};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {

    let cfg = Config::get_cfg().await?;

    let iota = Client::builder()
        .with_node(&cfg.node_url)?
        .finish()
        .await?;

    let balance = iota.get_address_balances(&cfg.addresses)
        .await?
        .get(0)
        .unwrap()
        .balance;

    println!("{balance:?}");

    Ok(())
}
