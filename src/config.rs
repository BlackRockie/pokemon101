#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub crypto_iso: String,
    pub fiat_iso: String,
    pub crypto_logo