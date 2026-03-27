use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use std::ops::Deref;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use todo::config::Config;
use todo::http::handlers::{create, delete_one, get_one, list, update};
use todo::http::modules::AppState;

use axum::{
    Router,
    routing::{get, patch, post},
};
use todo::auth::user::{create_user, login};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    dotenvy::dotenv().ok();

    let config = Config::parse();

    let client = redis::Client::open(config.redis_url.as_str()).unwrap();

    let pool = PgPoolOptions::new()
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres");

    let state = AppState {
        db: pool.clone(),
        secret: config.hmac_key,
        redis: client,
    };

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    let todo_router = Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(get_one).patch(update).delete(delete_one));

    let auth_router = Router::new()
        .route("/login", post(login))
        .route("/register", post(create_user))
        .route("/update", patch(update));

    let app = Router::new()
        .nest("/todo", todo_router)
        .nest("/auth", auth_router)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
