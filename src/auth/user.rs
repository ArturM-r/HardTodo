use crate::auth::extractor::AuthUser;
use crate::http::errors::AppError;
use crate::http::modules::AppState;
use anyhow::Result;
use argon2::{Argon2, PasswordHash, PasswordVerifier, password_hash::PasswordHasher};
use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::query;
use tracing::debug;

#[derive(Deserialize)]
pub struct NewUser {
    email: String,
    password: String,
}
#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct UserResponse {
    email: String,
    token: String,
}
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct UserUpdate {
    email: Option<String>,
    password: Option<String>,
}
pub async fn hash_password(password: String) -> Result<String, AppError> {
    let hash = tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_bytes())
            .map(|h| h.to_string())
    })
    .await
    .map_err(|err| {
        debug!("Join error {:?}", err);
        AppError::InternalError
    })?
    .map_err(|err| {
        debug!("{:?}", err);
        AppError::InternalError
    })?;

    Ok(hash)
}
pub async fn verify_password(password_hash: String, password: String) -> Result<bool, AppError> {
    let result = tokio::task::spawn_blocking(move || -> Result<bool, anyhow::Error> {
        let parsed_hash =
            PasswordHash::new(&password_hash).map_err(|_| anyhow::anyhow!("error hashing"))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    })
    .await
    .map_err(|err| {
        debug!("Join error {:?}", err);
        AppError::InternalError
    })?
    .map_err(|err| {
        debug!("{:?}", err);
        AppError::InternalError
    })?;
    Ok(result)
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<NewUser>,
) -> Result<Json<UserResponse>, AppError> {
    if req.password.is_empty() {
        Err(AppError::BadRequest("password is empty".to_string()))?
    }
    if req.email.is_empty() {
        Err(AppError::Unauthorized)?
    }
    let password_hash = hash_password(req.password)
        .await
        .map_err(|e| AppError::InternalError)?;

    let user_id = sqlx::query_scalar!(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) returning id",
        req.email,
        password_hash
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        debug!("database error: {}", err);
        AppError::Database(err)
    })?;
    Ok(Json(UserResponse {
        email: req.email,
        token: AuthUser { user_id }.to_jwt(&state),
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginUser>,
) -> Result<Json<UserResponse>, AppError> {
    let result = query!(
        "SELECT password_hash, id FROM users WHERE email = $1 ",
        req.email
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        debug!("database error {}", err);
        AppError::Database(err)
    })?;
    if verify_password(result.password_hash, req.password).await? == true {
        Ok(Json(UserResponse {
            email: req.email,
            token: AuthUser { user_id: result.id }.to_jwt(&state),
        }))
    } else {
        Err(AppError::NotFound)?
    }
}

pub async fn update(
    AuthUser { user_id }: AuthUser,
    State(state): State<AppState>,
    Json(req): Json<UserUpdate>,
) -> Result<Json<UserResponse>, AppError> {
    let hashed = if let Some(password) = req.password {
        Some(hash_password(password).await.map_err(|err| {
            debug!("Error update password where hashed: {}", err);
            AppError::BadRequest("failed to update password".into())
        })?)
    } else {
        None
    };
    let result = query!(
        "
        UPDATE users
        SET
            email = COALESCE($1, email),
            password_hash = COALESCE($2, password_hash)
        WHERE id = $3 returning email
        ",
        req.email,
        hashed,
        user_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        debug!("Error update password where hashed: {}", err);
        AppError::Database(err)
    })?;

    Ok(Json(UserResponse {
        email: result.email,
        token: AuthUser { user_id }.to_jwt(&state),
    }))
}
