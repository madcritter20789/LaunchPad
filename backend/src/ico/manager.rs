use crate::deposit::manager::{DepositManager, DepositStatus};
use crate::config::settings::LaunchpadConfig;
use chrono::Utc;
use std::error::Error;

pub struct IcoManager {
    config: LaunchpadConfig,
}

impl IcoManager {
    pub fn new() -> Self {
        IcoManager {
            config: LaunchpadConfig::new(),
        }
    }

    /// Retrieves the current ICO token price in USD.
    /// For this example, we simulate it as a constant value.
    async fn get_ico_token_price(&self) -> Result<f64, Box<dyn Error>> {
        // Example: ICO token price is 2 USD per token.
        Ok(2.0)
    }

    /// Starts the ICO monitoring process.
    /// Once the ICO period ends, iterates over all deposits marked as Released,
    /// converts their USD value into ICO tokens based on the current ICO token price,
    /// and simulates token distribution.
    pub async fn start(&self, deposit_manager: &DepositManager) -> Result<(), Box<dyn Error>> {
        let ico_end = self.config.ico_end;
        let now = Utc::now();
        let duration_until_ico_end = ico_end.signed_duration_since(now)
            .to_std()
            .unwrap_or(std::time::Duration::from_secs(0));

        println!(
            "ICO Manager started. ICO will end in {} seconds",
            duration_until_ico_end.as_secs()
        );

        let deposit_manager = deposit_manager.clone();
        let ico_chain = self.config.ico_chain.clone();

        tokio::spawn(async move {
            // Wait until the ICO period ends.
            tokio::time::sleep(duration_until_ico_end).await;
            println!("ICO period ended. Distributing tokens on chain: {}", ico_chain);

            let mut deposits = deposit_manager.deposits.lock().await;
            for (id, deposit) in deposits.iter_mut() {
                if deposit.status == DepositStatus::Released {
                    // Calculate the number of ICO tokens to allocate.
                    let tokens_allocated = deposit.usd_value / 2.0; // ICO token price simulated as 2 USD per token.
                    println!(
                        "Distributing {:.4} ICO tokens to {} for deposit {} (USD Value: {:.2})",
                        tokens_allocated, deposit.user_address, id, deposit.usd_value
                    );
                    // Here, you would invoke the ICO chain-specific transfer logic.
                    deposit.status = DepositStatus::Distributed;
                }
            }
        });

        Ok(())
    }
}
