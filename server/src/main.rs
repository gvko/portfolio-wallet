#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

use std::io::Error;
use std::u128;
use rocket::{Build, Rocket};
use rocket::serde::{Deserialize, Serialize, json::json, json::Json};
use reqwest::{Client, header};
use dotenv_codegen::dotenv;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct TokenBalancesApiResult {
    address: String,
    tokenBalances: Vec<TokenBalance>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct TokenInfoApiResult {
    decimals: i32,
    logo: Option<String>,
    name: String,
    symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct TokenBalance {
    contractAddress: String,
    tokenBalance: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct UserCoinBalance {
    balance: f64,
    name: String,
    symbol: String,
    logo: String,
}

#[get("/coins/<wallet_address>")]
async fn get_coins(wallet_address: &str) -> Result<Json<Vec<UserCoinBalance>>, Error> {
    let api_base_url = dotenv!("API_URL");
    let api_key = dotenv!("API_KEY");
    let api_url = format!("{}/{}", api_base_url, api_key);

    let wallet_address = wallet_address.to_string();

    let data = json!({
        "jsonrpc": "2.0",
        "method": "alchemy_getTokenBalances",
        "headers": {
            "Content-Type": "application/json",
        },
        "params": [format!("{}", wallet_address)]
    });

    let client = Client::new();
    let res = client
        .post(&api_url)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&data)
        .send()
        .await;

    let result = res.unwrap();
    let result = result.json::<serde_json::Value>().await.unwrap();
    let result: TokenBalancesApiResult = serde_json::from_value(result["result"].clone()).unwrap();

    let mut user_coins: Vec<UserCoinBalance> = vec![];

    for token_balance in &result.tokenBalances {
        let data = json!({
            "jsonrpc": "2.0",
            "method": "alchemy_getTokenMetadata",
            "headers": {
                "Content-Type": "application/json",
            },
            "params": [format!("{}", token_balance.contractAddress)]
        });

        let client = Client::new();
        let res = client
            .post(&api_url)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&data)
            .send()
            .await;

        let result = res.unwrap();
        let result = result.json::<serde_json::Value>().await.unwrap();
        let result = serde_json::from_value::<TokenInfoApiResult>(result["result"].clone())?;

        let logo = result.logo.clone().unwrap_or_else(|| "null".to_string());

        // println!("{}", token_balance.tokenBalance);
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

    Ok(Json(user_coins))
}

/// Main function of the Rocket framework.
/// Build the server instance and attach routes.
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![get_coins])
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();
    let _rocket = rocket().launch().await?;

    Ok(())
}