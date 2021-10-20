use cucumber_rust::{async_trait, Cucumber, World};
use reqwest;
use serde::{Deserialize};
use std::convert::Infallible;

mod steps;

#[derive(Deserialize, Debug)]
struct TimeResult {
    unixtime: i64,
    rfc1123: String,
}

#[derive(Deserialize, Debug)]
struct TimeResponse {
    error: Vec<String>,
    result: TimeResult,
}

#[derive(Deserialize, Debug)]
struct XBTUSDPair {
    altname: String,
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
struct AssetPairsResult {
    XXBTZUSD: XBTUSDPair,
}

#[derive(Deserialize, Debug)]
struct AssetPairsResponse {
    error: Vec<String>,
    result: AssetPairsResult,
}

struct Exchange {
    pub time: TimeResponse,
    pub trading_pair: AssetPairsResponse,
}

impl Exchange {
    async fn get_server_time(&mut self) {
        let request_url = format!("https://api.kraken.com/0/{scope}/{endpoint}",
                                  scope = "public",
                                  endpoint = "Time");
        let response = reqwest::get(&request_url).await.unwrap();
        self.time = response.json().await.unwrap();
    }

    async fn get_asset_pairs(&mut self) {
        let request_url = format!("https://api.kraken.com/0/{scope}/{endpoint}?{param1}={value1}",
                                    scope = "public",
                                    endpoint = "AssetPairs",
                                    param1 = "pair",
                                    value1 = "XBTUSD");
        let response = reqwest::get(&request_url).await.unwrap();
        self.trading_pair = response.json().await.unwrap();
    }
}

pub struct MyWorld {
    exchange: Exchange,
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
          exchange: Exchange {
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
          },
        })
    }
}

#[tokio::main]
async fn main() {

    Cucumber::<MyWorld>::new()
        // Specifies where our feature files exist
        .features(&["./tests/features"])
        // Adds the implementation of our steps to the runner
        .steps(steps::exchange::steps())
        // Add some global context for all the tests, like databases.
        //.context(Context::new().add(pool))
        // Add some lifecycle functions to manage our database nightmare
        //.before(feature("Example feature"), |ctx| {
        //    let pool = ctx.get::<SqlitePool>().unwrap().clone();
        //    async move { create_tables(&pool).await }.boxed()
        //})
        //.after(feature("Example feature"), |ctx| {
        //    let pool = ctx.get::<SqlitePool>().unwrap().clone();
        //    async move { drop_tables(&pool).await }.boxed()
        //})
        // Parses the command line arguments if passed
        .cli()
        // Runs the Cucumber tests and then exists
        .run_and_exit()
        .await
}
