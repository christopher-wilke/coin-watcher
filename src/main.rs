use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Ok, Result};

use coin_watcher::{
    config::{self, Config},
    state_manager::{BalanceState, StateManager},
};
use iota_client::Client;
use log::{info};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    console_subscriber::init();
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let state: StateManager = std::sync::Arc::new(tokio::sync::Mutex::new(HashMap::new()));
    let cfg = config::read_cfg().await?;

    loop {
        // Need to clone as we have a
        let state = state.clone();
        let cfg = cfg.clone();

        tokio::task::Builder::new()
            .name("get balance task")
            .spawn(async move {
                get_balance(cfg, state).await?;
                Ok(())
            })?;

        tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    }
}

async fn get_balance(cfg: Config, state: StateManager) -> Result<(), anyhow::Error> {
    // We need to instatiate the Client as it doesnt derive Copy or Clone
    let iota_client = Client::builder().with_node(&cfg.node_url)?.finish().await?;

    let resp = iota_client.get_address_balances(&cfg.addresses).await?;

    for balance_address_response in resp {
        let mut state = state.lock().await;
        state.insert(
            balance_address_response.address.clone(),
            BalanceState {
                balance: balance_address_response.balance,
                last_updated: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            },
        );

        info!("Updated State: {state:?}");
    }
    Ok(())
}
