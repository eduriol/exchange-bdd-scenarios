use chrono::{DateTime};
use cucumber_rust::{t, Steps};

pub fn steps() -> Steps<crate::MyWorld> {
    let mut steps: Steps<crate::MyWorld> = Steps::new();

    steps.when_async(
        "I request the server time",
        t!(|mut world, _ctx| {
            world.exchange.get_server_time().await;
            world
        }),
    );

    steps.then("I get a proper server time", |world, _ctx| {
        let server_time = DateTime::parse_from_str(
            &world.exchange.time.result.rfc1123, "%a, %d %b %y %T %z");
        assert!(server_time.is_ok());
        assert_eq!(server_time.unwrap().timestamp(), world.exchange.time.result.unixtime);
        world
    });

    steps.when_async(
        "I request the XBT/USD trading pair",
        t!(|mut world, _ctx| {
            world.exchange.get_asset_pairs().await;
            world
        }),
    );

    steps
}
