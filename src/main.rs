use std::{sync::{Arc}, time::{SystemTime, UNIX_EPOCH, Duration}, collections::HashMap};

use anyhow::{Result, Ok};

use coin_watcher::{config, state_manager::{BalanceState, StateManager}};
use iota_client::{
    Client,
};
use log::{info, debug, error};
use tokio::sync::Mutex;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {

    console_subscriber::init();
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let state: StateManager = std::sync::Arc::new(tokio::sync::Mutex::new(HashMap::new()));
    let cfg = config::read_cfg().await?;
    
    loop {
        let iota_client = Client::builder()
            .with_node(&cfg.node_url)?
            .finish()
            .await?;

        let adr = cfg.addresses.clone();
        let state = state.clone();

        tokio::task::Builder::new()
            .name("get balance task")
            .spawn(async move {
                get_balance(iota_client, &adr, state).await?;
                Ok(()) 
            })?;

        tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    }
}

async fn get_balance(
        iota_client: Client, 
        addresses: &[String],
        state: StateManager
    )
    -> Result<(), anyhow::Error>
{
    let resp = iota_client.get_address_balances(addresses).await?;

    for balance_address_response in resp {
        
            let mut state = state.lock().await;
            state.insert(balance_address_response.address.clone(), BalanceState {
                balance: balance_address_response.balance,
                last_updated: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
            });
        
        tokio::time::sleep(Duration::from_millis(5000)).await;
        info!("Updated State: {state:?}");
    }
    Ok(())
}