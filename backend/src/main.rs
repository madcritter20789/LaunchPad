use std::error::Error;
use tokio;
use warp::Filter;

mod chain;
mod deposit;
mod config;
mod wallet;
mod ico;

use deposit::manager::DepositManager;
use ico::manager::IcoManager;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Chain-Independent Cryptocurrency Launchpad System for ICO...");

    // Initialize and start the deposit manager.
    let deposit_manager = DepositManager::new();
    deposit_manager.start().await?;
    
    // Start the ICO manager.
    let ico_manager = IcoManager::new();
    ico_manager.start(&deposit_manager).await?;
    
    // Clone deposit_manager to pass it to warp filters.
    let dm_filter = warp::any().map(move || deposit_manager.clone());
    
    // GET / returns a welcome message.
    let root = warp::path::end().map(|| "Welcome to the ICO Launchpad API!");
    
    // GET /deposits returns JSON of all deposits.
    let deposits_route = warp::path("deposits")
        .and(warp::get())
        .and(dm_filter.clone())
        .and_then(handle_get_deposits);
    
    // POST /deposit creates a new deposit.
    let deposit_route = warp::path("deposit")
        .and(warp::post())
        .and(warp::body::json())
        .and(dm_filter.clone())
        .and_then(handle_create_deposit);
    
    let routes = root.or(deposits_route).or(deposit_route);
    
    println!("API server running on http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    
    Ok(())
}

async fn handle_get_deposits(deposit_manager: DepositManager) -> Result<impl warp::Reply, warp::Rejection> {
    let deposits = deposit_manager.get_deposits().await;
    Ok(warp::reply::json(&deposits))
}

#[derive(serde::Deserialize)]
struct NewDeposit {
    token_chain_id: String,
    token_contract_address: String,
    token_amount: String,
    user_address: String,
    // Lock duration in seconds.
    lock_duration: i64,
}

async fn handle_create_deposit(new_deposit: NewDeposit, deposit_manager: DepositManager) -> Result<impl warp::Reply, warp::Rejection> {
    use chrono::Duration;
    use crate::chain::interface::Token;
    
    let token = Token {
        chain_id: new_deposit.token_chain_id,
        contract_address: new_deposit.token_contract_address,
        amount: new_deposit.token_amount,
    };
    let lock_duration = Duration::seconds(new_deposit.lock_duration);
    
    match deposit_manager.create_deposit(token, new_deposit.user_address, lock_duration).await {
        Ok(deposit_id) => Ok(warp::reply::json(&json!({ "deposit_id": deposit_id }))),
        Err(e) => Ok(warp::reply::json(&json!({ "error": e.to_string() }))),
    }
}
