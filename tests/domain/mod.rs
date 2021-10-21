use cucumber_rust::{async_trait, World};
use reqwest;
use serde::{Deserialize};
use std::convert::Infallible;

#[derive(Deserialize, Debug)]
pub struct TimeResult {
    pub unixtime: i64,
    pub rfc1123: String,
}

#[derive(Deserialize, Debug)]
pub struct TimeResponse {
    error: Vec<String>,
    pub result: TimeResult,
}

#[derive(Deserialize, Debug)]
pub struct XBTUSDPair {
    pub altname: String,
    wsname: String,
    aclass_base: String,
    base: String,
    aclass_quote: String,
    quote: String,
    lot: String,
    pair_decimals: u32,
    lot_decimals: u32,
    lot_multiplier: u32,
    leverage_buy: Vec<u32>,
    leverage_sell: Vec<u32>,
    fees: Vec<Vec<f32>>,
    fees_maker: Vec<Vec<f32>>,
    fee_volume_currency: String,
    margin_call: u32,
    margin_stop: u32,
    ordermin: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AssetPairsResult {
    pub XXBTZUSD: XBTUSDPair,
}

#[derive(Deserialize, Debug)]
pub struct AssetPairsResponse {
    error: Vec<String>,
    pub result: AssetPairsResult,
}

pub struct ExchangeWorld {
    pub time: TimeResponse,
    pub trading_pair: AssetPairsResponse,
}

impl ExchangeWorld {
    pub async fn get_server_time(&mut self) {
        let request_url = format!("https://api.kraken.com/0/{scope}/{endpoint}",
                                           scope = "public",
                                           endpoint = "Time");
        let response = reqwest::get(&request_url).await.unwrap();
        self.time = response.json().await.unwrap();
    }

    pub async fn get_asset_pairs(&mut self) {
        let request_url = format!("https://api.kraken.com/0/{scope}/{endpoint}?{param1}={value1}",
                                           scope = "public",
                                           endpoint = "AssetPairs",
                                           param1 = "pair",
                                           value1 = "XBTUSD");
        let response = reqwest::get(&request_url).await.unwrap();
        self.trading_pair = response.json().await.unwrap();
    }
}

#[async_trait(?Send)]
impl World for ExchangeWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            time: TimeResponse {
                error: vec![],
                result: TimeResult {
                    unixtime: 0,
                    rfc1123: "Thu, 1 Jan 1970 00:00:00 +0000".to_string(),
                },
            },
            trading_pair: AssetPairsResponse {
                error: vec![],
                result: AssetPairsResult {
                    XXBTZUSD: XBTUSDPair {
                        altname: "".to_string(),
                        wsname: "".to_string(),
                        aclass_base: "".to_string(),
                        base: "".to_string(),
                        aclass_quote: "".to_string(),
                        quote: "".to_string(),
                        lot: "".to_string(),
                        pair_decimals: 0,
                        lot_decimals: 0,
                        lot_multiplier: 0,
                        leverage_buy: vec![],
                        leverage_sell: vec![],
                        fees: vec![],
                        fees_maker: vec![],
                        fee_volume_currency: "".to_string(),
                        margin_call: 0,
                        margin_stop: 0,
                        ordermin: "".to_string()
                    }
                }
            }
        })
    }
}
