use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainConfig {
    pub rpc_url: String,
    pub chain_id: String,
    pub native_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchpadConfig {
    pub chains: HashMap<String, ChainConfig>,
    pub minimum_lock_duration: u64,
    pub maximum_lock_duration: u64,
}

impl LaunchpadConfig {
    pub fn new() -> Self {
        let mut chains = HashMap::new();
        chains.insert(
            "ethereum".to_string(),
            ChainConfig {
                rpc_url: "https://ethereum-rpc.example.com".to_string(),
                chain_id: "1".to_string(),
                native_token: "ETH".to_string(),
            },
        );

        LaunchpadConfig {
            chains,
            minimum_lock_duration: 86400,    // 1 day in seconds
            maximum_lock_duration: 2592000,   // 30 days in seconds
        }
    }
}