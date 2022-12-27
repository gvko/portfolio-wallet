use crate::alchemy_api::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCoinBalance {
    pub balance: f64,
    pub name: String,
    pub symbol: String,
    pub logo: String,
}

pub async fn get_coins_for_address(api_url: String, wallet_address: String) -> Vec<UserCoinBalance> {
    let result = get_balances(&api_url, wallet_address).await;
    let mut user_coins: Vec<UserCoinBalance> = vec![];

    for token_balance in &result.tokenBalances {
        let result = get_tokens_metadata(&api_url, &token_balance.contractAddress).await;
        let logo = result.logo.clone().unwrap_or("null".to_string());

        let hexnum = token_balance.tokenBalance.trim_start_matches("0x");
        let balance_as_float = u128::from_str_radix(hexnum, 16).unwrap() as f64;
        let balance = balance_as_float / 10_f64.powi(result.decimals);
        let balance = balance.round() / 100.0;

        let current_coin_balance = UserCoinBalance {
            balance,
            name: result.name,
            symbol: result.symbol,
            logo,
        };
        user_coins.push(current_coin_balance);
    }

    user_coins
}