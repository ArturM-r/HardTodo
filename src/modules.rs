    use std::sync::Arc;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use crate::db::Db;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Todo {
        pub id: uuid::Uuid,
        pub title: String,
        pub completed: bool,
        pub created_at: DateTime<Utc>,
    }
    #[derive(Deserialize)]
    pub struct TodoCreate{
        pub title: String,
    }
    #[derive(Deserialize)]
    pub struct TodoUpdate{
        pub title: Option<String>,
        pub completed: Option<bool>,
    }
    #[derive(Deserialize)]
    pub struct TodoDelete{
        pub id: uuid::Uuid,
    }
    #[derive(Serialize)]
    pub struct TodoResponse {
        pub id: uuid::Uuid,
        pub title: String,
        pub completed: bool,
        pub time: DateTime<chrono::Utc>,
    }

    pub struct AppState {
        pub db: Db
    }


    impl From<Todo> for  TodoResponse{
        fn from(value: Todo) -> Self {
            Self {
                id: value.id,
                title: value.title,
                completed: value.completed,
                time: value.created_at,
            }
        }
    }

