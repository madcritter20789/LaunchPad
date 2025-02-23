use std::error::Error;
use tokio;

mod chain;
mod deposit;
mod config;
mod wallet;

use deposit::manager::DepositManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Cross-Chain Launchpad...");
    
    let deposit_manager = DepositManager::new();
    deposit_manager.start().await?;
    
    Ok(())
}