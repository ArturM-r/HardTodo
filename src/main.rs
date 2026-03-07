mod handlers;
mod db;
mod modules;

use std::sync::Arc;
use axum;
use axum::extract::State;
use axum::Router;
use axum::routing::{delete, get, post, put};
use tokio::net::TcpListener;
use crate::db::Db;
use crate::handlers::{create, delete_one, get_all, get_one, update};
use crate::modules::AppState;



#[tokio::main]
async fn main() {
    let db = Db::new();
    let state = Arc::new(AppState{db });

    let app: Router<()> = Router::new()
        .route("/todo", get(get_all))
        .route("/todo", post(create))
        .route("/todo/{id}", get(get_one))
        .route("/todo/{id}", put(update))
        .route("/todo/{id}", delete(delete_one))
        .with_state(state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}