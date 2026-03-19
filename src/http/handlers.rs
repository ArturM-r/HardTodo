use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    auth::extractor::AuthUser,
    http::modules::AppState,
    http::{
        db,
        errors::AppError,
        modules::{TodoCreate, TodoResponse, TodoUpdate},
    },
};

pub async fn create(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Json(req): Json<TodoCreate>,
) -> Result<StatusCode, AppError> {
    db::create(
        &state.db,
        user_id,
        req.title,
        req.completed.unwrap_or(false),
    )
    .await
    .map_err(|_| AppError::BadRequest("failed to create todo".into()))?;

    Ok(StatusCode::CREATED)
}

pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    AuthUser { user_id }: AuthUser,
) -> Result<Json<TodoResponse>, AppError> {
    let todo = db::get(&state.db, user_id, id)
        .await
        .map_err(|_| AppError::NotFound)?;

    Ok(Json(todo))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    AuthUser { user_id }: AuthUser,
    Json(req): Json<TodoUpdate>,
) -> Result<StatusCode, AppError> {
    db::update(&state.db, user_id, id, req.title, req.completed)
        .await
        .map_err(|_| AppError::NotFound)?;

    Ok(StatusCode::OK)
}

pub async fn delete_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    AuthUser { user_id }: AuthUser,
) -> Result<StatusCode, AppError> {
    db::delete(&state.db, user_id, id)
        .await
        .map_err(|_| AppError::NotFound)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn list(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
) -> Result<Json<Vec<TodoResponse>>, AppError> {
    let todos = db::list(&state.db, user_id)
        .await
        .map_err(|_| AppError::BadRequest("failed to fetch todos".into()))?;

    Ok(Json(todos))
}
