use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ChainConfig {
    pub rpc_url: String,
    pub chain_id: String,
    pub native_token: String,
}

#[derive(Debug)]
pub struct LaunchpadConfig {
    pub chains: HashMap<String, ChainConfig>,
    pub ico_start: DateTime<Utc>,
    pub ico_end: DateTime<Utc>,
    pub ico_chain: String, // The blockchain where the ICO token is created/distributed.
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
        chains.insert(
            "bitcoin".to_string(),
            ChainConfig {
                rpc_url: "https://bitcoin-rpc.example.com".to_string(),
                chain_id: "BTC".to_string(),
                native_token: "BTC".to_string(),
            },
        );

        // For demonstration, set ICO to start now and end 2 minutes later.
        let now = Utc::now();
        let ico_start = now;
        let ico_end = now + Duration::seconds(120);

        LaunchpadConfig {
            chains,
            ico_start,
            ico_end,
            ico_chain: "ethereum".to_string(), // ICO tokens will be distributed on Ethereum.
            minimum_lock_duration: 86400,    // 1 day in seconds (adjustable)
            maximum_lock_duration: 2592000,   // 30 days in seconds
        }
    }
}
