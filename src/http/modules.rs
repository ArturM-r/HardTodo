use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::types::PgLQueryVariantFlag;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
pub struct QueryFilter {
    pub offset: u32,
    pub limit: u32,
    pub search: Option<String>,
    pub completed: Option<bool>,
}
#[derive(Deserialize)]
pub struct Queryfr {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub completed: Option<bool>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct TodoCreate {
    pub title: String,
    pub completed: Option<bool>,
}

#[derive(Deserialize)]
pub struct TodoUpdate {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct TodoDelete {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Default, FromRow)]
pub struct TodoResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub secret: String,
}

impl From<Todo> for TodoResponse {
    fn from(value: Todo) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            title: value.title,
            completed: value.completed,
            created_at: value.created_at,
        }
    }
}
impl From<Queryfr> for QueryFilter {
    fn from(query: Queryfr) -> Self {
        Self {
            offset: query.offset.unwrap_or(0),
            limit: query.limit.unwrap_or(10),
            search: query.search,
            completed: query.completed,
        }
    }
}
