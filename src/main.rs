
use todo::config::Config;
use todo::http;
use todo::auth;

use crate::handlers::{create, delete_one, get_all, get_one, update};
use crate::modules::AppState;
use axum;
use axum::Router;
use axum::routing::{delete, get, post, put};
use dotenv::dotenv;
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    dotenv().ok();

    let Config = Config::parse();

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");
    let state = Arc::new(AppState { db: pool, secret: secret_key });

    let app: Router<()> = Router::new()
        .route("/todo", get(get_all))
        .route("/todo", post(create))
        .route("/todo/{id}", get(get_one))
        .route("/todo/{id}", put(update))
        .route("/todo/{id}", delete(delete_one))
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    info!("Server shutting down");
}
