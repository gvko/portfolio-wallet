#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

use std::io::Error;
use rocket::{Build, Rocket};
use rocket::serde::json::json;
use reqwest::{Client, header};
use dotenv_codegen::dotenv;

#[get("/coins/<wallet_address>")]
async fn get_coins(wallet_address: &str) -> Result<(), Error> {
    let API_URL = dotenv!("API_URL");
    let API_KEY = dotenv!("API_KEY");
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
        .post(format!("{}/{}", API_URL, API_KEY))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&data)
        .send()
        .await;

    let result = response.unwrap();
    let result = result.json::<serde_json::Value>().await.unwrap();
    let result = result["result"].clone();

    println!("Result: {}", result);

    Ok(())
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