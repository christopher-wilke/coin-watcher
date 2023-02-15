use std::{sync::{Arc}, collections::HashMap};

use tokio::sync::Mutex;

#[derive(Debug)]
pub struct BalanceState {
    pub balance: u64,
    pub last_updated: u64
}

pub type StateManager = Arc<Mutex<HashMap<String, BalanceState>>>;