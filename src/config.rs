use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(long, env)]
    pub database_url: String,

    #[arg(long, env)]
    pub hmac_key: String,
}