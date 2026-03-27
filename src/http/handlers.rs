use crate::http::modules::{QueryFilter, Queryfr};
use crate::{
    auth::extractor::AuthUser,
    http::modules::AppState,
    http::{
        db,
        errors::AppError,
        modules::{TodoCreate, TodoResponse, TodoUpdate},
    },
};
use axum::extract::Query;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use redis::AsyncTypedCommands;
use redis::aio::MultiplexedConnection;
use tracing::debug;
use uuid::Uuid;
pub async fn invalidate_cache(
    user_id: &Uuid,
    connection: &mut MultiplexedConnection,
) -> Result<(), redis::RedisError> {
    let mut cursor: u64 = 0;
    let mut keys_delete = Vec::new();
    let pattern = format!("todos:{}*", user_id);

    loop {
        let (next_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
            .arg(cursor)
            .arg("MATCH")
            .arg(&pattern)
            .arg("COUNT")
            .arg(100)
            .query_async(connection)
            .await?;
        cursor = next_cursor;
        keys_delete.extend(keys);
        if cursor == 0 {
            break;
        }
    }
    if !keys_delete.is_empty() {
        connection.del(&keys_delete).await?;
    }
    Ok(())
}
pub async fn get_redis_conn(state: &AppState) -> Result<MultiplexedConnection, AppError> {
    state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|err| {
            debug!("redis connect error: {}", err);
            AppError::InternalError
        })
}

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
    .await?;
    let mut connection = get_redis_conn(&state).await?;
    invalidate_cache(&user_id, &mut connection)
        .await
        .map_err(|err| {
            debug!("invalidate cache error{}", err);
            AppError::InternalError
        })?;
    Ok(StatusCode::CREATED)
}

pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    AuthUser { user_id }: AuthUser,
) -> Result<Json<TodoResponse>, AppError> {
    let todo = db::get(&state.db, user_id, id).await?;

    Ok(Json(todo))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    AuthUser { user_id }: AuthUser,
    Json(req): Json<TodoUpdate>,
) -> Result<StatusCode, AppError> {
    db::update(&state.db, user_id, id, req.title, req.completed).await?;
    let mut connection = get_redis_conn(&state).await?;
    invalidate_cache(&user_id, &mut connection)
        .await
        .map_err(|err| {
            debug!("invalidate cache error{}", err);
            AppError::InternalError
        })?;

    Ok(StatusCode::OK)
}

pub async fn delete_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    AuthUser { user_id }: AuthUser,
) -> Result<StatusCode, AppError> {
    db::delete(&state.db, user_id, id).await?;
    let mut connection = get_redis_conn(&state).await?;
    invalidate_cache(&user_id, &mut connection)
        .await
        .map_err(|err| {
            debug!("invalidate cache error{}", err);
            AppError::InternalError
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<Queryfr>,
    AuthUser { user_id }: AuthUser,
) -> Result<Json<Vec<TodoResponse>>, AppError> {
    let mut connection = get_redis_conn(&state).await?;
    let key = format!(
        "todos:{user_id}:limit:{}:offset:{}:completed:{}:search:{}",
        params.limit.unwrap_or(0),
        params.offset.unwrap_or(0),
        params.completed.unwrap_or(false),
        params.search.as_deref().unwrap_or(""),
    );

    let cached = connection.get(&key).await.map_err(|err| {
        debug!("redis key search error{}", err);
        AppError::InternalError
    })?;

    if let Some(cached_json) = cached {
        let todos: Vec<TodoResponse> =
            serde_json::from_str(&cached_json).map_err(|_| AppError::InternalError)?;
        return Ok(Json(todos));
    }

    let filter = QueryFilter::from(params);
    let todos = db::list(&state.db, user_id, &filter).await?;

    let json = serde_json::to_string(&todos).map_err(|_| AppError::InternalError)?;

    connection.set_ex(&key, &json, 60).await.map_err(|err| {
        debug!("redis error{}", err);
        AppError::InternalError
    })?;

    Ok(Json(todos))
}
