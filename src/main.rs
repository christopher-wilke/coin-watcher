use std::{sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}, collections::HashMap};

use anyhow::Result;

use coin_watcher::{config, state_manager::{BalanceState, StateManager}};
use iota_client::{
    Client,
};
use log::{info, debug, error};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let cfg = config::read_cfg().await?;
    let state: StateManager = Arc::new(Mutex::new(HashMap::new()));

    let iota = Client::builder()
        .with_node(&cfg.node_url)?
        .finish()
        .await?;
    
    loop {
        let resp = iota.get_address_balances(&cfg.addresses).await?;

        for balance_address_response in resp {
            
            let mut state = state.lock().unwrap();
            state.insert(balance_address_response.address.clone(), BalanceState {
                balance: balance_address_response.balance,
                last_updated: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
            });
        }

        tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    }

    Ok(())
}
