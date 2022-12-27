#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

mod alchemy_api;
mod coins_service;

use std::io::Error;
use std::u128;
use rocket::{Build, Rocket};
use rocket::serde::{Deserialize, Serialize, json::Json};
use dotenv_codegen::dotenv;
use coins_service::*;

#[get("/coins/<wallet_address>")]
async fn get_coins(wallet_address: &str) -> Result<Json<Vec<UserCoinBalance>>, Error> {
    let api_base_url = dotenv!("API_URL");
    let api_key = dotenv!("API_KEY");
    let api_url = format!("{}/{}", api_base_url, api_key);
    let wallet_address = wallet_address.to_string();

    let user_coins = get_coins_for_address(api_url, wallet_address).await;

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