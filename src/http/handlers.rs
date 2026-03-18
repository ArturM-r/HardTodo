use crate::auth::extractor::AuthUser;
use crate::http::errors::AppError;
use crate::http::modules::{AppState, TodoCreate, TodoDelete, TodoResponse, TodoUpdate};
use axum::http::StatusCode;
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::query;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    AuthUser { user_id }: AuthUser,
) -> Result<Json<TodoResponse>, AppError> {
    let result = query!(
        "SELECT id, user_id, title, completed, created_at FROM todos WHERE id = $1 AND user_id = $2",
        id, user_id
    ).fetch_one(&state.db).await.map_err(|e| {
        AppError::NotFound
    })?;

    Ok(Json(TodoResponse {
        id: result.id,
        user_id,
        title: result.title,
        completed: result.completed,
        time: result.created_at,
    }))
}

pub async fn create(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Json(req): Json<TodoCreate>,
) -> Result<(StatusCode, Json<TodoResponse>), AppError> {
    let result = query!(
        "INSERT INTO todos (user_id, title) VALUES ($1, $2) returning  created_at, id",
        user_id,
        req.title
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| AppError::BadRequest("CANNOT CREATE".to_string()))?;
    Ok((
        StatusCode::OK,
        Json(TodoResponse {
            id: result.id,
            user_id,
            title: req.title,
            time: result.created_at,
            completed: false,
        }),
    ))
}

pub async fn delete_one(
    Path(delete): Path<TodoDelete>,
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
) -> Result<(StatusCode, Json<TodoDelete>), AppError> {
    let result = query!(
        "DELETE FROM todos WHERE id = $1 AND user_id = $2 RETURNING id",
        delete.id,
        user_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| AppError::NotFound)?;

    Ok((StatusCode::OK, Json(TodoDelete { id: result.id })))
}

pub async fn update(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<TodoUpdate>,
) -> Result<StatusCode, AppError> {
    let result = query!(
        "UPDATE todos
        SET
            title = COALESCE($1, title),
            completed = COALESCE($2, completed)
        WHERE id = $3 and user_id = $4 ",
        req.title,
        req.completed,
        id,
        user_id
    )
    .execute(&state.db)
    .await
    .map_err(|_| AppError::NotFound)?;
    Ok(StatusCode::OK)
}
