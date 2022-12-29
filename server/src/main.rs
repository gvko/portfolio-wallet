#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

mod alchemy_api;
mod tokens_service;
mod middlewares;

use tokens_service::*;
use middlewares::CORS;
use std::io::Error;
use dotenv_codegen::dotenv;
use rocket::{
    Build, Rocket,
    serde::{json::Json},
};

#[get("/tokens/<wallet_address>")]
async fn get_tokens(wallet_address: &str) -> Result<Json<Vec<UserTokenBalance>>, Error> {
    let api_base_url = dotenv!("API_URL");
    let api_key = dotenv!("API_KEY");
    let api_url = format!("{}/{}", api_base_url, api_key);
    let wallet_address = wallet_address.to_string();

    let user_tokens = get_tokens_for_address(api_url, wallet_address).await;

    Ok(Json(user_tokens))
}

/// Main function of the Rocket framework.
/// Build the server instance and attach routes.
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![get_tokens])
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();

    let _rocket = rocket()
        .attach(CORS)
        .launch()
        .await?;

    Ok(())
}