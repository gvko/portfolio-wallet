use rocket::serde::{Deserialize, Serialize, json::json};
use reqwest::{Client, header};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TokenBalancesApiResult {
    pub address: String,
    pub tokenBalances: Vec<TokenBalance>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TokenBalance {
    pub contractAddress: String,
    pub tokenBalance: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TokenInfoApiResult {
    pub decimals: i32,
    pub logo: Option<String>,
    pub name: String,
    pub symbol: String,
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

pub async fn get_balances(api_url: &String, wallet_address: String) -> TokenBalancesApiResult {
    let result: TokenBalancesApiResult = make_request(
        &api_url,
        "alchemy_getTokenBalances".to_string(),
        format!("{}", wallet_address),
    ).await;
    result
}

pub async fn get_tokens_metadata(api_url: &String, contract_address: &String) -> TokenInfoApiResult {
    let result: TokenInfoApiResult = make_request(
        &api_url,
        "alchemy_getTokenMetadata".to_string(),
        format!("{}", contract_address),
    ).await;
    result
}