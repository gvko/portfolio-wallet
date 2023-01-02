use crate::alchemy_api::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserNft {
    pub name: String,
    pub description: String,
    pub image: String,
    pub id: u32,
    pub date: u64,
}

pub async fn get_nfts_for_address(api_url: String, wallet_address: String) -> Vec<UserNft> {
    let nfts = get_nfts(&api_url, wallet_address).await;
    let mut user_nfts: Vec<UserNft> = vec![];

    for nft in &nfts.ownedNfts {
        user_nfts.push(UserNft {
            name: nft.title.clone(),
            description: nft.description.clone(),
            image: nft.media[0].thumbnail.clone(),
            id: nft.metadata.edition,
            date: nft.metadata.date,
        });
    }

    user_nfts
}