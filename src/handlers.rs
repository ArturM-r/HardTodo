use crate::errors::AppError;
use crate::modules::{AppState, TodoCreate, TodoResponse, TodoUpdate};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub async fn get_one(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<TodoResponse>, AppError> {
    let todo = state.db.get(id).ok_or(AppError::NotFound)?;
    info!(%id, "fetched todo");
    Ok(Json(todo.into()))
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TodoCreate>,
) -> Result<StatusCode, AppError> {
    if payload.title.trim().is_empty() {
        return Err(AppError::BadRequest("title cannot be empty".into()));
    }

    state.db.create(payload.title);
    info!("created todo");
    Ok(StatusCode::CREATED)
}

pub async fn get_all(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TodoResponse>>, AppError> {
    let todos: Vec<TodoResponse> = state.db.get_all().into_iter().map(Into::into).collect();
    info!("fetched all todos");
    Ok(Json(todos))
}

pub async fn delete_one(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, AppError> {
    if state.db.delete(id) {
        info!(%id, "deleted todo");
        Ok(StatusCode::NO_CONTENT)
    } else {
        info!(%id, "todo not found");
        Err(AppError::NotFound)
    }
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<TodoUpdate>,
) -> Result<StatusCode, AppError> {
    if let Some(title) = &payload.title {
        if title.trim().is_empty() {
            return Err(AppError::BadRequest("title cannot be empty".into()));
        }
    }

    if state.db.update(id, payload.title, payload.completed) {
        info!(%id, "updated todo");
        Ok(StatusCode::OK)
    } else {
        info!(%id, "todo not found");
        Err(AppError::NotFound)
    }
}