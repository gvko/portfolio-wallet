use rocket::serde::{Deserialize, Serialize, json::{json, to_string}};
use reqwest::{Client, header};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct TokenBalancesApiResult {
    pub address: String,
    pub tokenBalances: Vec<TokenBalanceApiObj>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct TokenBalanceApiObj {
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct NftInfoApiResult {
    pub blockHash: String,
    pub totalCount: u16,
    pub ownedNfts: Vec<NftApiObject>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct NftApiObject {
    pub title: String,
    pub description: String,
    pub media: Vec<NftApiObjMedia>,
    pub metadata: NftApiObjMetadata,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct NftApiObjMedia {
    pub raw: String,
    pub gateway: String,
    pub thumbnail: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct NftApiObjMetadata {
    pub date: u64,
    pub image: String,
    pub name: String,
    pub description: String,
    pub edition: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct TransactionsApiResult {
    pub transfers: Vec<TransactionApiObj>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct TransactionApiObj {
    pub asset: String,
    pub blockNum: String,
    pub category: String,
    pub from: String,
    pub to: String,
    pub hash: String,
    pub value: f64,
}

struct Endpoints;

impl Endpoints {
    const GET_TOKEN_BALANCES: &'static str = "alchemy_getTokenBalances";
    const GET_TOKEN_METADATA: &'static str = "alchemy_getTokenMetadata";
    const GET_NFTS: &'static str = "getNFTs";
    const GET_TRANSACTIONS: &'static str = "alchemy_getAssetTransfers";
}

pub struct Network;

impl Network {
    pub const ETH: &'static str = "Ethereum";
    pub const POLYGON: &'static str = "Polygon";
}

pub struct Asset;

impl Asset {
    const TOKEN: &'static str = "token";
    const NFT: &'static str = "nft";
}

const API_URL_BASE_PREFIX: &str = dotenv!("API_URL_BASE_PREFIX");
const API_URL_PREFIX_ETH: &str = dotenv!("API_URL_PREFIX_ETH");
const API_URL_PREFIX_POLYGON: &str = dotenv!("API_URL_PREFIX_POLYGON");
const API_URL_SUFFIX_TOKEN: &str = dotenv!("API_URL_SUFFIX_TOKEN");
const API_URL_SUFFIX_NFT: &str = dotenv!("API_URL_SUFFIX_NFT");
const API_KEY_ETH: &str = dotenv!("API_KEY_ETH");
const API_KEY_POLYGON: &str = dotenv!("API_KEY_POLYGON");

/// Make an RPC request to a given endpoint, parse and return the JSON response
///
/// # Parameters
/// * `api_url`: A string slice representing the base URL of the API to which the request should be made.
/// * `endpoint`: A string slice representing the endpoint of the API to which the request should be made.
/// * `params`: A string representing the request payload to be included in the request.
///
/// # Returns
/// The deserialized response from the API as the specified type `T`.
///
/// # Example
/// ```
/// let response: MyResponseType = make_post_request(&"api_url", "my_endpoint", "some_value").await;
/// ```
async fn make_rpc_request<T, G>(api_url: &String, endpoint: &str, params: Vec<G>) -> T
    where T: for<'a> Deserialize<'a>,
          G: Serialize {
    let data = json!({
        "jsonrpc": "2.0",
        "method": endpoint,
        "headers": {
            "Content-Type": "application/json",
        },
        "params": params
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

/// Make an HTTP GET request to a given endpoint, parse and return the JSON response
///
/// # Parameters
/// * `api_url`: A string slice representing the base URL of the API to which the request should be made.
/// * `endpoint`: A string slice representing the endpoint of the API to which the request should be made.
/// * `params`: A tuple of strings representing the query parameters to be included in the request.
///
/// # Returns
/// The deserialized response from the API as the specified type `T`.
///
/// # Example
/// ```
/// let response: MyResponseType = make_get_request(&"api_url", "my_endpoint", ("param1", "value1")).await;
/// ```
async fn make_get_request<T>(api_url: &String, endpoint: &str, params: (String, String)) -> T where T: for<'a> Deserialize<'a> {
    let client = Client::new();
    let url = format!("{}/{}", api_url, endpoint);
    let res = client
        .get(url)
        .query(&[params])
        .send()
        .await;

    let result = res.unwrap();
    let result = result.json::<serde_json::Value>().await.unwrap();
    let result: T = serde_json::from_value(result).unwrap();
    result
}

/// Constructs a URL for the token endpoint of a particular network.
///
/// # Parameters
/// * `network`: A string slice representing the network for which the URL should be constructed.
///
/// # Returns
/// A string representing the constructed URL.
///
/// # Example
/// ```
/// let url = construct_url_tokens("ETH");
/// ```
fn construct_api_url(network: &str, asset: &str) -> String {
    let (network_string, api_key) = match network {
        Network::ETH => (API_URL_PREFIX_ETH.to_string(), API_KEY_ETH.to_string()),
        Network::POLYGON => (API_URL_PREFIX_POLYGON.to_string(), API_KEY_POLYGON.to_string()),
        _ => todo!()
    };
    let asset_suffix = match asset {
        Asset::TOKEN => API_URL_SUFFIX_TOKEN.to_string(),
        Asset::NFT => API_URL_SUFFIX_NFT.to_string(),
        _ => todo!()
    };
    format!("{}{}{}/{}", API_URL_BASE_PREFIX, network_string, asset_suffix, api_key)
}

/// Get a list of tokens owned by a given address
///
/// # Parameters
/// * `network`: A string slice representing the network on which the token is located.
/// * `wallet_address`: A string slice representing the wallet address for which we search.
///
/// # Returns
/// A `TokenBalancesApiResult` object representing the response from the API.
///
/// # Example
/// ```
/// let token_balances = get_token_balances("ETH", "0x1234567890abcdef").await;
/// ```
pub async fn get_token_balances(network: &str, wallet_address: String) -> TokenBalancesApiResult {
    let url = construct_api_url(network, Asset::TOKEN);
    let result: TokenBalancesApiResult = make_rpc_request(
        &url,
        Endpoints::GET_TOKEN_BALANCES,
        vec![wallet_address],
    ).await;
    result
}

/// Get the metadata for a given token by its contract address
///
/// # Parameters
/// * `network`: A string slice representing the network on which the token is located.
/// * `contract_address`: A string slice representing the contract address of the token.
///
/// # Returns
/// A `TokenInfoApiResult` object representing the response from the API.
///
/// # Example
/// ```
/// let token_metadata = get_tokens_metadata("ETH", "0x1234567890abcdef").await;
/// ```
pub async fn get_tokens_metadata(network: &str, contract_address: &String) -> TokenInfoApiResult {
    let url = construct_api_url(network, Asset::TOKEN);
    let result: TokenInfoApiResult = make_rpc_request(
        &url,
        Endpoints::GET_TOKEN_METADATA,
        vec![contract_address],
    ).await;
    result
}

/// Get a list of NFTs owned by a given address
///
/// # Parameters
/// * `network`: A string slice representing the network on which the token is located.
/// * `contract_address`: A string slice representing the contract address of the token.
///
/// # Returns
/// A `NftInfoApiResult` object representing the response from the API.
///
/// # Example
/// ```
/// let nfts = get_nfts("ETH", "0x1234567890abcdef").await;
/// ```
pub async fn get_nfts(network: &str, wallet_address: String) -> NftInfoApiResult {
    let url = construct_api_url(network, Asset::NFT);
    let result: NftInfoApiResult = make_get_request(
        &url,
        Endpoints::GET_NFTS,
        ("owner".to_string(), wallet_address),
    ).await;
    result
}

/// Get a list of transactions for a given address
///
/// # Parameters
/// * `network`: A string slice representing the network on which the token is located.
/// * `wallet_address`: A string slice representing the wallet address for which we search.
///
/// # Returns
/// A `TokenBalancesApiResult` object representing the response from the API.
///
/// # Example
/// ```
/// let transactions = get_wallet_transactions("ETH", "0x1234567890abcdef").await;
/// ```
pub async fn get_wallet_transactions(network: &str, wallet_address: String) -> TransactionsApiResult {
    let url = construct_api_url(network, Asset::TOKEN);
    let params = json!({
        "fromAddress": wallet_address,
        "fromBlock": "0xF1EB1D", // TODO: make this dynamically populated
        "toBlock": "latest",
        "category": [
            "external",
            "internal",
            "erc20",
            "erc721",
            "erc1155"
        ],
        "withMetadata": false,
        "excludeZeroValue": true,
        "maxCount": "0x3e8",
        "order": "desc"
    });

    let result: TransactionsApiResult = make_rpc_request(
        &url,
        Endpoints::GET_TRANSACTIONS,
        vec![params],
    ).await;
    result
}