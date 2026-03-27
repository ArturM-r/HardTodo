use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(long, env = "DATABASE_URL")]
    pub database_url: String,

    #[arg(long, env = "HMAC_KEY")]
    pub hmac_key: String,

    #[arg(long, env = "REDIS_URL")]
    pub redis_url: String,
}