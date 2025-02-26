use std::error::Error;
use tokio;

mod chain;
mod deposit;
mod config;
mod wallet;
mod ico;

use deposit::manager::DepositManager;
use ico::manager::IcoManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Chain-Independent Cryptocurrency Launchpad System for ICO...");

    // Initialize the deposit manager to handle incoming user deposits.
    let deposit_manager = DepositManager::new();
    deposit_manager.start().await?;

    // Initialize the ICO manager which is responsible for distributing tokens on the ICO chain.
    let ico_manager = IcoManager::new();
    ico_manager.start(&deposit_manager).await?;

    // Keep the main process alive.
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
