use clap::Parser;
use dotenv::dotenv;
use log::{info, log};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use todo::config::Config;
use todo::http::handlers::{create, delete_one, get_one, update, list};
use todo::http::modules::AppState;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use axum::routing::patch;
use tracing::callsite::register;
use todo::auth::user::{create_user, login};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    dotenv().ok();

    let config = Config::parse();

    let pool = PgPoolOptions::new()
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres");

    let state = AppState {
        db: pool,
        secret: config.hmac_key,
    };

    let app = Router::new()
        .route("/todo", get(list))
        .route("/todo", post(create))
        .route("/todo/{id}", get(get_one))
        .route("/todo/{id}", put(update))
        .route("/todo/{id}", delete(delete_one))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let auth = Router::new()
        .route("/login", get(login))
        .route("/register", post(create_user))
        .route("/update", patch(update));
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
