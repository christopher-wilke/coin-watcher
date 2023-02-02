use std::{sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}, collections::HashMap};

use anyhow::Result;

use coin_watcher::{config, state_manager::{BalanceState, StateManager}};
use iota_client::{
    Client,
};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {

    let cfg = config::read_cfg().await?;
    let state: StateManager = Arc::new(Mutex::new(HashMap::new()));

    let iota = Client::builder()
        .with_node(&cfg.node_url)?
        .finish()
        .await?;
    
    loop {
        let balance = iota.get_address_balances(&cfg.addresses)
            .await?
            .get(0)
            .unwrap()
            .balance;

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        println!("Since Unix Epoch: {now}");

        println!("Current balance is: {balance:?}i");    
        tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    }

    Ok(())
}
