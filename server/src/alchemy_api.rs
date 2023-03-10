use reqwest::{header, Client};
use rocket::serde::{json::json, Deserialize, Serialize};
use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use thiserror::Error as ThisError;

pub type SerdeError = serde_json::Error;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct TokenBalances {
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

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TokenInfo {
    pub decimals: i32,
    pub logo: Option<String>,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct OwnedNftList {
    pub blockHash: String,
    pub totalCount: u16,
    pub ownedNfts: Vec<NftObject>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct NftObject {
    pub title: String,
    pub description: String,
    pub media: Vec<NftMedia>,
    pub metadata: NftMetadata,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct NftMedia {
    pub raw: String,
    pub gateway: String,
    pub thumbnail: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct NftMetadata {
    pub date: u64,
    pub image: String,
    pub name: String,
    pub description: String,
    pub edition: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct TransactionsList {
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

#[derive(ThisError, Debug)]
pub enum AlchemyApiError {
    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("RPC Error: {0}")]
    RPCError(String),
    #[error("RequestError: {0}")]
    RequestError(#[from] reqwest::Error),
}

impl From<AlchemyApiError> for Error {
    fn from(err: AlchemyApiError) -> Self {
        Error::new(ErrorKind::Other, err)
    }
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
async fn make_rpc_request<T, G>(
    api_url: &String,
    endpoint: &str,
    params: Vec<G>,
) -> Result<T, AlchemyApiError>
where
    T: for<'a> Deserialize<'a>,
    G: Serialize + Debug,
{
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
        .await
        .map_err(AlchemyApiError::RequestError)?;

    let api_result: serde_json::Value = res.json().await.map_err(AlchemyApiError::RequestError)?;

    if let Some(error) = api_result.get("error") {
        let err_msg = error["message"].as_str().unwrap_or("Unknown API error");
        return Err(AlchemyApiError::RPCError(err_msg.to_owned()));
    }

    let result_deserialized = serde_json::from_value::<T>(api_result["result"].clone())
        .map_err(AlchemyApiError::SerdeError);
    match result_deserialized {
        Ok(result) => Ok(result),
        Err(err) => {
            error!("Could not deserialize the Alchemy API response. Endpoint: {endpoint}, Params: {params:?}");
            //TODO: remove println statements
            println!("{:#?}", err.to_string());
            println!("{api_result:#?}");
            Err(err)
        }
    }
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
async fn make_get_request<T>(
    api_url: &String,
    endpoint: &str,
    params: (String, String),
) -> Result<T, AlchemyApiError>
where
    T: for<'a> Deserialize<'a>,
{
    let url = format!("{api_url}/{endpoint}");
    let client = Client::new();
    let res = client
        .get(url)
        .query(&[&params])
        .send()
        .await
        .map_err(AlchemyApiError::RequestError)?;

    let api_result: serde_json::Value = res.json().await.map_err(AlchemyApiError::RequestError)?;

    if let Some(error) = api_result.get("error") {
        let err_msg = error["message"].as_str().unwrap_or("Unknown API error");
        return Err(AlchemyApiError::RPCError(err_msg.to_owned()));
    }

    let result_deserialized =
        serde_json::from_value::<T>(api_result.clone()).map_err(AlchemyApiError::SerdeError);
    match result_deserialized {
        Ok(result) => Ok(result),
        Err(err) => {
            error!("Could not deserialize the Alchemy API response. Endpoint: {endpoint}, Params: {params:?}");
            //TODO: remove println statements
            println!("{:#?}", err.to_string());
            println!("{api_result:#?}");
            Err(err)
        }
    }
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
        Network::POLYGON => (
            API_URL_PREFIX_POLYGON.to_string(),
            API_KEY_POLYGON.to_string(),
        ),
        // default to ETH network
        _ => (API_URL_PREFIX_ETH.to_string(), API_KEY_ETH.to_string()),
    };
    let asset_suffix = match asset {
        Asset::TOKEN => API_URL_SUFFIX_TOKEN.to_string(),
        Asset::NFT => API_URL_SUFFIX_NFT.to_string(),
        // default to tokens
        _ => API_URL_SUFFIX_TOKEN.to_string(),
    };
    format!("{API_URL_BASE_PREFIX}{network_string}{asset_suffix}/{api_key}")
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
pub async fn get_token_balances(
    network: &str,
    wallet_address: String,
) -> Result<TokenBalances, AlchemyApiError> {
    let url = construct_api_url(network, Asset::TOKEN);
    let result: TokenBalances =
        make_rpc_request(&url, Endpoints::GET_TOKEN_BALANCES, vec![wallet_address]).await?;
    Ok(result)
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
pub async fn get_tokens_metadata(
    network: &str,
    contract_address: &String,
) -> Result<TokenInfo, AlchemyApiError> {
    let url = construct_api_url(network, Asset::TOKEN);
    let result: TokenInfo =
        make_rpc_request(&url, Endpoints::GET_TOKEN_METADATA, vec![contract_address]).await?;
    Ok(result)
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
pub async fn get_nfts(
    network: &str,
    wallet_address: String,
) -> Result<OwnedNftList, AlchemyApiError> {
    let url = construct_api_url(network, Asset::NFT);
    let result: OwnedNftList = make_get_request(
        &url,
        Endpoints::GET_NFTS,
        ("owner".to_string(), wallet_address),
    )
    .await?;
    Ok(result)
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
pub async fn get_wallet_transactions(
    network: &str,
    wallet_address: String,
) -> Result<TransactionsList, AlchemyApiError> {
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

    let result: TransactionsList =
        make_rpc_request(&url, Endpoints::GET_TRANSACTIONS, vec![params]).await?;
    Ok(result)
}
