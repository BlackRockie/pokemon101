
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate toml;

mod config;

#[derive(Serialize, Deserialize, Debug)]
struct Crypto {
    ticker: Ticker,
    success: bool,
    error: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Ticker {
    base: String,
    target: String,
    price: String,
    volume: String,
    change: String 
}

fn main() {
    let config = config::get_config();

    let resp: Result<Crypto, reqwest::Error> = make_request(create_request_url(config.crypto_iso, config.fiat_iso));

    match resp {