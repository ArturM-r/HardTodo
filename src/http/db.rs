use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::http::modules::TodoResponse;

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    title: String,
    completed: bool,
) -> Result<(), sqlx::Error> {
    let result = query!(
        r#"
        INSERT INTO todos (user_id, title, completed)
        VALUES ($1, $2, $3)
        "#,
        user_id,
        title,
        completed
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 1 {
        Ok(())
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}

pub async fn get(pool: &PgPool, user_id: Uuid, id: Uuid) -> Result<TodoResponse, sqlx::Error> {
    let todo = query_as!(
        TodoResponse,
        r#"
        SELECT
            id,
            user_id,
            title,
            completed,
            created_at AS time
        FROM todos
        WHERE user_id = $1 AND id = $2
        "#,
        user_id,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn list(pool: &PgPool, user_id: Uuid) -> Result<Vec<TodoResponse>, sqlx::Error> {
    let todos = query_as!(
        TodoResponse,
        r#"
        SELECT
            id,
            user_id,
            title,
            completed,
            created_at AS time
        FROM todos
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

pub async fn update(
    pool: &PgPool,
    user_id: Uuid,
    id: Uuid,
    title: Option<String>,
    completed: Option<bool>,
) -> Result<(), sqlx::Error> {
    let result = query!(
        r#"
        UPDATE todos
        SET
            title = COALESCE($1, title),
            completed = COALESCE($2, completed)
        WHERE user_id = $3 AND id = $4
        "#,
        title,
        completed,
        user_id,
        id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 1 {
        Ok(())
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}

pub async fn delete(pool: &PgPool, user_id: Uuid, id: Uuid) -> Result<(), sqlx::Error> {
    let result = query!(
        r#"
        DELETE FROM todos
        WHERE user_id = $1 AND id = $2
        "#,
        user_id,
        id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 1 {
        Ok(())
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}
