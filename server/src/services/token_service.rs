use crate::alchemy_api::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserTokenBalance {
    pub balance: f64,
    pub name: String,
    pub symbol: String,
    pub logo: String,
}

pub async fn get_tokens_for_address(wallet_address: String) -> Result<Vec<UserTokenBalance>, SerdeError> {
    let tokens = get_token_balances(Network::ETH, wallet_address).await?;
    let mut user_tokens: Vec<UserTokenBalance> = vec![];

    for token_balance in &tokens.tokenBalances {
        let result = get_tokens_metadata(Network::ETH, &token_balance.contractAddress).await?;
        let logo = result.logo.clone().unwrap_or("null".to_string());

        let hexnum = token_balance.tokenBalance.trim_start_matches("0x");
        let balance_as_float = u128::from_str_radix(hexnum, 16).unwrap() as f64;
        let balance = balance_as_float / 10_f64.powi(result.decimals);
        let balance = balance.round() / 100.0;

        let current_token_balance = UserTokenBalance {
            balance,
            name: result.name,
            symbol: result.symbol,
            logo,
        };
        user_tokens.push(current_token_balance);
    }

    Ok(user_tokens)
}
