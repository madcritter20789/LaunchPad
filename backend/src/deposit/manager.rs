use crate::chain::interface::Token;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum DepositStatus {
    Active,
    Released,
    Distributed,
    Failed,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Deposit {
    pub token: Token,
    pub user_address: String,
    pub lock_until: DateTime<Utc>,
    pub status: DepositStatus,
    // The USD value of the deposit computed at the time of deposit.
    pub usd_value: f64,
}

#[derive(Clone)]
pub struct DepositManager {
    pub deposits: Arc<Mutex<HashMap<String, Deposit>>>,
}

impl DepositManager {
    pub fn new() -> Self {
        DepositManager {
            deposits: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Starts the deposit manager's background task to monitor deposits.
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Deposit Manager started");
        self.monitor_deposits().await?;
        Ok(())
    }

    /// Creates a new deposit for an ICO contribution.
    /// It converts the deposited token amount to USD using a simulated cross-chain conversion rate.
    pub async fn create_deposit(
        &self,
        token: Token,
        user_address: String,
        lock_duration: chrono::Duration,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Convert token amount (string) to f64.
        let amount: f64 = token.amount.parse()?;
        // Retrieve the conversion rate for the deposit's chain.
        let conversion_rate = Self::get_conversion_rate(&token.chain_id, &token).await?;
        let usd_value = amount * conversion_rate;

        let deposit_id = Uuid::new_v4().to_string();
        let deposit = Deposit {
            token,
            user_address,
            lock_until: Utc::now() + lock_duration,
            status: DepositStatus::Active,
            usd_value,
        };

        let mut deposits = self.deposits.lock().await;
        deposits.insert(deposit_id.clone(), deposit);

        Ok(deposit_id)
    }

    /// Returns a clone of the current deposits.
    pub async fn get_deposits(&self) -> HashMap<String, Deposit> {
        let deposits = self.deposits.lock().await;
        deposits.clone()
    }

    /// Periodically monitors deposits.
    /// Once the lock period expires, marks deposits as Released.
    async fn monitor_deposits(&self) -> Result<(), Box<dyn std::error::Error>> {
        let deposits = self.deposits.clone();

        tokio::spawn(async move {
            loop {
                let mut deposits = deposits.lock().await;
                let now = Utc::now();

                for (id, deposit) in deposits.iter_mut() {
                    if deposit.status == DepositStatus::Active && deposit.lock_until <= now {
                        deposit.status = DepositStatus::Released;
                        println!("Deposit {} is now released for ICO participation", id);
                    }
                }

                drop(deposits);
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        });

        Ok(())
    }

    /// Simulated cross-chain conversion rate function.
    /// Returns the USD conversion rate for a given chain.
    async fn get_conversion_rate(chain_id: &str, _token: &Token) -> Result<f64, Box<dyn std::error::Error>> {
        match chain_id {
            "ethereum" => Ok(2000.0), // 1 ETH = 2000 USD
            "bitcoin"  => Ok(30000.0), // 1 BTC = 30000 USD
            _ => Ok(100.0), // Default conversion rate
        }
    }
}
