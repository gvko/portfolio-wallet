use crate::alchemy_api::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserNfts {}

pub async fn get_nfts_for_address(api_url: String, wallet_address: String) -> Vec<UserNfts> {
    let nfts = get_nfts(&api_url, wallet_address).await;
    let user_nfts: Vec<UserNfts> = vec![];

    user_nfts
}