use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Token {
    pub chain_id: String,
    pub contract_address: String,
    pub amount: String, // Represented as a string (e.g. "1.5")
}

#[async_trait]
pub trait ChainInterface {
    async fn deposit(&self, token: Token) -> Result<String, Box<dyn std::error::Error>>;
    async fn withdraw(&self, user_address: &str, amount: String) -> Result<(), Box<dyn std::error::Error>>;
}
