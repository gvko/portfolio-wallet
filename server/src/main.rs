#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

use std::io::Error;
use rocket::{Build, Rocket};
use rocket::serde::{Deserialize, Serialize, json::json, json::Json};
use reqwest::{Client, header};
use dotenv_codegen::dotenv;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct AlchemyApiResult {
    address: String,
    tokenBalances: Vec<TokenBalance>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct TokenBalance {
    contractAddress: String,
    tokenBalance: String,
}

#[get("/coins/<wallet_address>")]
async fn get_coins(wallet_address: &str) -> Result<Json<Vec<TokenBalance>>, Error> {
    let api_url = dotenv!("API_URL");
    let api_key = dotenv!("API_KEY");
    let wallet_address = wallet_address.to_string();

    let data = json!({
        "jsonrpc": "2.0",
        "method": "alchemy_getTokenBalances",
        "headers": {
            "Content-Type": "application/json",
        },
        "params": [format!("{}", wallet_address)],
        "id": 42,
    });

    let client = Client::new();
    let response = client
        .post(format!("{}/{}", api_url, api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&data)
        .send()
        .await;

    let result = response.unwrap();
    let result = result.json::<serde_json::Value>().await.unwrap();
    let result: AlchemyApiResult = serde_json::from_value(result["result"].clone()).unwrap();

    println!("{:?}", result);

    Ok(Json(result.tokenBalances))
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