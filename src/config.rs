#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub crypto_iso: String,
    pub fiat_iso: String,
    pub crypto_logo: String
}

pub fn get_config() -> Config {
    let crypto_iso = String::from("btc");
    let fia