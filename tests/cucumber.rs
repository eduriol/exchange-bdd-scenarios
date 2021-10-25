use cucumber_rust::Cucumber;

mod domain;
mod steps;

#[tokio::main]
async fn main() {
    Cucumber::<crate::domain::ExchangeWorld>::new()
        .features(&["./tests/features"])
        .steps(steps::steps())
        .run_and_exit()
        .await
}
