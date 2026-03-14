use crate::modules::Todo;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;
pub async fn create(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    title: String,
    completed: bool,
) -> Result<(), sqlx::Error> {
    let result = query!(
        "INSERT INTO todos (id, user_id, title, completed) VALUES ($1, $2, $3, $4)",
        id,
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
pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    let result = query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 1 {
        Ok(())
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}
pub async fn delete_todo(pool: &PgPool, user_id: Uuid, id: Uuid) -> Result<(), sqlx::Error> {
    let result = query!(
        "DELETE FROM todos WHERE user_id = $1 AND id = $2",
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

pub async fn get(pool: &PgPool, user_id: Uuid) -> Result<Vec<Todo>, sqlx::Error> {
    let result = query_as!(Todo, "SELECT * FROM todos WHERE user_id = $1", user_id)
        .fetch_all(pool)
        .await?;
    Ok(result)
}
pub async fn update(
    pool: &PgPool,
    title: Option<String>,
    completed: Option<bool>,
    user_id: Uuid,
    id: Uuid,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        "
        UPDATE todos
        SET
            title = COALESCE($1, title),
            completed = COALESCE($2, completed)
        WHERE id = $3 AND user_id = $4
        ",
        title,
        completed,
        id,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 1 {
        Ok(())
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}
