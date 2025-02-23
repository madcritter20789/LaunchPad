use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletAddress {
    pub chain: String,
    pub address: String,
}

#[async_trait]
pub trait WalletHandler {
    async fn validate_address(&self, address: &str) -> Result<bool, Box<dyn std::error::Error>>;
    async fn transfer(&self, from: &WalletAddress, to: &WalletAddress, amount: String) -> Result<String, Box<dyn std::error::Error>>;
}