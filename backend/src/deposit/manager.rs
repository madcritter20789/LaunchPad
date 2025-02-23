use crate::chain::interface::Token;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct Deposit {
    pub token: Token,
    pub user_address: String,
    pub lock_until: DateTime<Utc>,
    pub status: DepositStatus,
}

#[derive(Debug, PartialEq)]
pub enum DepositStatus {
    Active,
    Released,
    Failed,
}

pub struct DepositManager {
    deposits: Arc<Mutex<HashMap<String, Deposit>>>,
}

impl DepositManager {
    pub fn new() -> Self {
        DepositManager {
            deposits: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Deposit Manager started");
        self.monitor_deposits().await?;
        Ok(())
    }

    pub async fn create_deposit(
        &self,
        token: Token,
        user_address: String,
        lock_duration: chrono::Duration,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let deposit_id = Uuid::new_v4().to_string();
        let deposit = Deposit {
            token,
            user_address,
            lock_until: Utc::now() + lock_duration,
            status: DepositStatus::Active,
        };

        let mut deposits = self.deposits.lock().await;
        deposits.insert(deposit_id.clone(), deposit);

        Ok(deposit_id)
    }

    async fn monitor_deposits(&self) -> Result<(), Box<dyn std::error::Error>> {
        let deposits = self.deposits.clone();
        
        tokio::spawn(async move {
            loop {
                let mut deposits = deposits.lock().await;
                let now = Utc::now();

                for (id, deposit) in deposits.iter_mut() {
                    if deposit.status == DepositStatus::Active && deposit.lock_until <= now {
                        deposit.status = DepositStatus::Released;
                        println!("Released deposit {}", id);
                    }
                }

                drop(deposits);
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        });

        Ok(())
    }
}