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

struct Exchange {
    pub time: TimeResponse,
}

impl Exchange {
    async fn get_server_time(&mut self) {
        let request_url = format!("https://api.kraken.com/0/{scope}/{endpoint}",
                                  scope = "public",
                                  endpoint = "Time");
        let response = reqwest::get(&request_url).await.unwrap();
        self.time = response.json().await.unwrap();
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
          exchange: Exchange{ time: TimeResponse{
              error: vec![],
              result: TimeResult {
                  unixtime: 0,
                  rfc1123: "Thu, 1 Jan 70 00:00:00 +0000".to_string(),
              },
          }},
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
