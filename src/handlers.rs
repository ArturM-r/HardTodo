use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Error, Json};
use axum::http::StatusCode;
use uuid::Uuid;
use crate::modules::{AppState, Todo, TodoCreate, TodoResponse, TodoUpdate};

pub async fn get_one(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<Json<TodoResponse>, StatusCode> {
    let todo = state.db.get(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(todo.into()))
}
pub async fn create(State(state): State<Arc<AppState>>, Json(payload): Json<TodoCreate>) -> StatusCode {
    state.db.create(payload.title);
    StatusCode::OK
}
pub async fn get_all(State(state): State<Arc<AppState>>) -> Result<Json<Vec<TodoResponse>>, StatusCode> {
    let todo: Vec<TodoResponse>= state.db.get_all().into_iter().map(|x| {
        x.into()
    }).collect();
    Ok(Json(todo))
}
pub async fn delete_one(Path(id): Path<Uuid>, State(state): State<Arc<AppState>>) -> StatusCode {
    if state.db.delete(id){
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
pub async fn  update(State(state): State<Arc<AppState>>,Path(id): Path<Uuid> ,Json(payload): Json<TodoUpdate>, ) -> StatusCode {
    if state.db.update(id, payload.title, payload.completed){
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}