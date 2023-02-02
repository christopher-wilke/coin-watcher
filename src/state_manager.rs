use std::{sync::{Arc, Mutex}, collections::HashMap};

pub struct BalanceState {
    balance: u64,
    last_updated: u64
}

// {"addr": BalanceState {}}
pub type StateManager = Arc<Mutex<HashMap<String, BalanceState>>>;