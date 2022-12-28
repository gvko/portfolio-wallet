#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

mod alchemy_api;
mod coins_service;

use std::io::Error;
use dotenv_codegen::dotenv;
use rocket::{
    Build, Rocket, Request, Response,
    http::{Header},
    serde::{json::Json},
};
use rocket::fairing::{Fairing, Info, Kind};
use coins_service::*;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

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

    let _rocket = rocket()
        .attach(CORS)
        .launch()
        .await?;

    Ok(())
}