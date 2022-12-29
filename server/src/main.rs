#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

mod alchemy_api;
mod services;
mod middlewares;

use middlewares::CORS;
use services::{token_service, nft_service};
use std::io::Error;
use dotenv_codegen::dotenv;
use rocket::{
    Build, Rocket,
    serde::{json::Json},
};

const API_URL_ETH_TOKEN: &str = dotenv!("API_URL_ETH_TOKEN");
const API_URL_ETH_NFT: &str = dotenv!("API_URL_ETH_NFT");
const API_KEY: &str = dotenv!("API_KEY");

#[get("/tokens/<wallet_address>")]
async fn get_tokens(wallet_address: &str) -> Result<Json<Vec<token_service::UserTokenBalance>>, Error> {
    let api_url = format!("{}/{}", API_URL_ETH_TOKEN, API_KEY);
    let wallet_address = wallet_address.to_string();

    let user_tokens = token_service::get_tokens_for_address(api_url, wallet_address).await;
    Ok(Json(user_tokens))
}

#[get("/nfts/<wallet_address>")]
async fn get_nfts(wallet_address: &str) -> Result<Json<Vec<nft_service::UserNfts>>, Error> {
    let api_url = format!("{}/{}", API_URL_ETH_TOKEN, API_KEY);
    let wallet_address = wallet_address.to_string();

    let user_nfts = nft_service::get_nfts_for_address(api_url, wallet_address).await;
    Ok(Json(user_nfts))
}

/// Main function of the Rocket framework.
/// Build the server instance and attach routes.
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![
            get_tokens,
            get_nfts
        ])
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