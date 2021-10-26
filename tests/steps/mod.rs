use crate::domain::OrderSet;
use chrono::{DateTime, Utc};
use cucumber_rust::{t, Steps};
use data_encoding::BASE64;
use hmac::{Hmac, Mac, NewMac};
use sha2::{Digest, Sha256, Sha512};
use std::env;

pub fn steps() -> Steps<crate::domain::ExchangeWorld> {
    let mut steps: Steps<crate::domain::ExchangeWorld> = Steps::new();

    steps.when_async(
        "I request the server time",
        t!(|mut world, _ctx| {
            world.get_server_time().await;
            world
        }),
    );

    steps.then("I get a proper server time", |world, _ctx| {
        // Check that the server time has the expected rfc1123 format
        let server_time =
            DateTime::parse_from_str(&world.time.result.rfc1123, "%a, %d %b %y %T %z");
        assert!(server_time.is_ok());
        // Check that the informed unixtime corresponds to the rfc1123 time
        assert_eq!(server_time.unwrap().timestamp(), world.time.result.unixtime);
        world
    });

    steps.when_async(
        "I request the XBT/USD trading pair",
        t!(|mut world, _ctx| {
            world.get_asset_pairs().await;
            world
        }),
    );

    steps.then("I get proper trading pair info", |world, _ctx| {
        // Check that the altname is the expected one to validate trading pair info
        assert_eq!(world.trading_pair.result.XXBTZUSD.altname, "XBTUSD");
        world
    });

    steps.given("I have a 2FA account", |mut world, _ctx| {
        // Set all the values required to later perform the authenticated request
        world.auth_info.one_time_password = env::var("OTP").unwrap();
        world.auth_info.api_key = env::var("API_KEY").unwrap();
        world.auth_info.api_nonce = (DateTime::timestamp(&Utc::now()) * 1000000).to_string();
        world.auth_info.api_sign = get_api_signature(
            "/0/private/OpenOrders".to_string(),
            "nonce=".to_owned()
                + &world.auth_info.api_nonce
                + "&otp="
                + world.auth_info.one_time_password.as_str(),
            env::var("API_SECRET").unwrap(),
            world.auth_info.api_nonce.to_owned(),
        );
        world
    });

    steps.when_async(
        "I request the open orders",
        t!(|mut world, _ctx| {
            world.get_open_orders().await;
            world
        }),
    );

    steps.then("I get my list of open orders", |world, _ctx| {
        // Check that the response contains a list of Open Orders, although an empty one
        assert_eq!(world.open_orders.result.open, OrderSet {});
        world
    });

    steps
}

/// Generates API Signature value required for private calls.
fn get_api_signature(api_endpoint: String, data: String, secret: String, nonce: String) -> String {
    // Decode secret
    let api_secret = BASE64.decode(secret.as_bytes()).unwrap();
    // Generate SHA-256 from request data
    let mut api_sha256 = Sha256::new();
    api_sha256.update((nonce + data.as_str()).as_bytes());
    let mut api_sha256 = api_sha256.finalize().to_vec();
    // Generate API_SIGN as HMAC-SHA-512 based on API url and previous SHA-256
    let mut sign_data = api_endpoint.as_bytes().to_vec();
    sign_data.append(&mut api_sha256);
    type HmacSha512 = Hmac<Sha512>;
    let mut api_sign = HmacSha512::new_from_slice(&api_secret[..]).unwrap();
    api_sign.update(&sign_data);
    BASE64.encode(&api_sign.finalize().into_bytes()[..])
}
