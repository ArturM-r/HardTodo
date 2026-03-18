use crate::http::errors::AppError;
use crate::http::modules::AppState;
use axum::extract::FromRequestParts;
use axum::http::HeaderValue;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use jsonwebtoken;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tracing::debug;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}
#[derive(Serialize, Deserialize)]
struct AuthUserClaims {
    user_id: Uuid,
    exp: u64,
}

impl AuthUser {
    pub fn to_jwt(&self, state: &AppState) -> String {
        let token = jsonwebtoken::encode(
            &Header::default(),
            &AuthUserClaims {
                user_id: self.user_id,
                exp: (OffsetDateTime::now_utc() + Duration::hours(24)).unix_timestamp() as u64,
            },
            &EncodingKey::from_secret(state.secret.as_ref()),
        );
        token.unwrap()
    }
    pub fn from_jwt(state: &AppState, header: &HeaderValue) -> Result<Self, AppError> {
        let data = header.to_str().map_err(|_| {
            debug!("Header is wrong");
            AppError::UNAUTHORIZED
        })?;

        if !data.starts_with("Bearer") {
            debug!(
                "Authorization header is using the wrong scheme: {:?}",
                header
            );
            return Err(AppError::UNAUTHORIZED);
        }
        let token = &data["Bearer ".len()..];

        let token_data = jsonwebtoken::decode::<AuthUserClaims>(
            token,
            &DecodingKey::from_secret(state.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| AppError::UNAUTHORIZED)?;
        if token_data.claims.exp < OffsetDateTime::now_utc().unix_timestamp() as u64 {
            info!("Token is out of data");
            return Err(AppError::UNAUTHORIZED);
        };
        Ok(AuthUser {
            user_id: token_data.claims.user_id,
        })
    }
}
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(AppError::UNAUTHORIZED)?;

        Self::from_jwt(state, auth_header)
    }
}
