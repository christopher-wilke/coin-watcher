use std::{sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}, collections::HashMap};

use anyhow::{Result, Ok};

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
    
    loop {
        let iota_client = Client::builder()
            .with_node(&cfg.node_url)?
            .finish()
            .await?;

        let adr = cfg.addresses.clone();
        let state = state.clone();
        tokio::spawn(async move {
            get_balance(iota_client, &adr, state).await?;
            Ok(())
        });
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
       
        let mut state = state.lock().unwrap();
        state.insert(balance_address_response.address.clone(), BalanceState {
            balance: balance_address_response.balance,
            last_updated: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
        });

        info!("Updated State: {state:?}");
    }
    Ok(())
}