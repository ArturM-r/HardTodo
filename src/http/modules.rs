use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub user_id: Uuid,
    pub id: uuid::Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}
#[derive(Deserialize)]
pub struct TodoCreate {
    pub title: String,
}
#[derive(Deserialize)]
pub struct TodoUpdate {
    pub title: Option<String>,
    pub completed: Option<bool>,
}
#[derive(Deserialize, Serialize)]
pub struct TodoDelete {
    pub id: uuid::Uuid,
}
#[derive(Serialize, Default)]
pub struct TodoResponse {
    pub id: Uuid,
    pub user_id: uuid::Uuid,
    pub title: String,
    pub completed: bool,
    pub time: DateTime<Utc>,
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
            user_id: value.id,
            title: value.title,
            completed: value.completed,
            time: value.created_at,
        }
    }
}
