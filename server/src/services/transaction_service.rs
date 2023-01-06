use crate::alchemy_api::*;
use rocket::serde::{Deserialize, Serialize};
use crate::get_transactions;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserTransactions {
    pub balance: f64,
    pub name: String,
    pub symbol: String,
    pub logo: String,
}

pub async fn get_transactions_for_address(wallet_address: String) -> Vec<UserTransactions> {
    let transactions = get_wallet_transactions(Network::ETH, wallet_address).await;
    let mut user_transactions: Vec<UserTransactions> = vec![];

    user_transactions
}