use cucumber_rust::Cucumber;

mod domain;
mod steps;

#[tokio::main]
async fn main() {

    Cucumber::<crate::domain::ExchangeWorld>::new()
        // Specifies where our feature files exist
        .features(&["./tests/features"])
        // Adds the implementation of our steps to the runner
        .steps(steps::steps())
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
