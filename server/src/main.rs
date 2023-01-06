#[macro_use]
extern crate rocket;
#[macro_use]
extern crate dotenv_codegen;

mod alchemy_api;
mod services;
mod middlewares;

use middlewares::CORS;
use services::{token_service, nft_service, transaction_service};
use std::io::Error;
use rocket::{
    Build, Rocket,
    serde::{json::Json},
};

#[get("/tokens/<wallet_address>")]
async fn get_tokens(wallet_address: &str) -> Result<Json<Vec<token_service::UserTokenBalance>>, Error> {
    let wallet_address = wallet_address.to_string();

    let user_tokens = token_service::get_tokens_for_address(wallet_address).await;
    Ok(Json(user_tokens))
}

#[get("/nfts/<wallet_address>")]
async fn get_nfts(wallet_address: &str) -> Result<Json<Vec<nft_service::UserNft>>, Error> {
    let wallet_address = wallet_address.to_string();

    let user_nfts = nft_service::get_nfts_for_address(wallet_address).await;
    Ok(Json(user_nfts))
}

#[get("/transactions/<wallet_address>")]
async fn get_transactions(wallet_address: &str) -> Result<Json<Vec<transaction_service::UserTransaction>>, Error> {
    let wallet_address = wallet_address.to_string();

    let user_transactions = transaction_service::get_transactions_for_address(wallet_address).await;
    Ok(Json(user_transactions))
}

/// Main function of the Rocket framework.
/// Build the server instance and attach routes.
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![
            get_tokens,
            get_nfts,
            get_transactions
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