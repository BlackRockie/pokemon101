
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
        Err(e) => handler(e),
        Ok(resp) => print_crypto(config.crypto_logo, format_price(resp.ticker.price), format_target_currency(resp.ticker.target)),
    }
}

fn create_request_url(crypto_iso: String, fiat_iso: String) -> String {
    return format!("https://api.cryptonator.com/api/ticker/{}-{}", &crypto_iso, &fiat_iso);
}