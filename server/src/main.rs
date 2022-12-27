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

async fn make_request<T>(api_url: &String, endpoint: String, params: String) -> T where T: for<'a> Deserialize<'a> {
    let data = json!({
        "jsonrpc": "2.0",
        "method": endpoint,
        "headers": {
            "Content-Type": "application/json",
        },
        "params": [params]
    });

    let client = Client::new();
    let res = client
        .post(api_url)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&data)
        .send()
        .await;

    let result = res.unwrap();
    let result = result.json::<serde_json::Value>().await.unwrap();
    let result: T = serde_json::from_value(result["result"].clone()).unwrap();
    result
}

async fn get_balances(api_url: &String, wallet_address: String) -> TokenBalancesApiResult {
    let result: TokenBalancesApiResult = make_request(
        &api_url,
        "alchemy_getTokenBalances".to_string(),
        format!("{}", wallet_address),
    ).await;
    result
}

async fn get_tokens_metadata(api_url: &String, contract_address: &String) -> TokenInfoApiResult {
    let result: TokenInfoApiResult = make_request(
        &api_url,
        "alchemy_getTokenMetadata".to_string(),
        format!("{}", contract_address),
    ).await;
    result
}
// TODO: make function for making request to endpoint for token info

#[get("/coins/<wallet_address>")]
async fn get_coins(wallet_address: &str) -> Result<Json<Vec<UserCoinBalance>>, Error> {
    let api_base_url = dotenv!("API_URL");
    let api_key = dotenv!("API_KEY");
    let api_url = format!("{}/{}", api_base_url, api_key);
    let wallet_address = wallet_address.to_string();

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