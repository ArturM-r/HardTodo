use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(long, env)]
    pub database_url: String,

    #[arg(long, env)]
    pub hmac_key: String,
}
pub struct TodoConfig {
    pub database_url: String,
    pub hmac_key: String,
}