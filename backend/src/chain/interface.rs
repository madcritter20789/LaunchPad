use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Token {
    pub chain_id: String,
    pub contract_address: String,
    pub amount: String,
}

#[async_trait]
pub trait ChainInterface {
    async fn deposit(&self, token: Token) -> Result<String, Box<dyn std::error::Error>>;
    async fn withdraw(&self, transaction_id: String) -> Result<(), Box<dyn std::error::Error>>;
}