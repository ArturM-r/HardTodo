use crate::http::errors::AppError;
use crate::http::modules::{QueryFilter, Queryfr, TodoResponse};
use sqlx::{PgPool, Postgres, QueryBuilder, query, query_as};
use uuid::Uuid;

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
            created_at
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

pub async fn list(
    pool: &PgPool,
    user_id: Uuid,
    filter: &QueryFilter,
) -> Result<Vec<TodoResponse>, AppError> {
    if !(1..100).contains(&filter.limit) {
        return Err(AppError::Database(sqlx::Error::InvalidArgument(
            "limit to big".to_string(),
        )));
    }
    let mut builder = QueryBuilder::<Postgres>::new(
        "SELECT id, user_id, title, completed, created_at FROM todos WHERE user_id = ",
    );
    builder.push_bind(user_id);
    if let Some(completed) = filter.completed {
        builder.push(" AND completed = ");
        builder.push_bind(completed);
    }
    if let Some(search) = &filter.search {
        let search = search.trim();
        if !search.is_empty() {
            builder.push(" AND title ILIKE ");
            builder.push_bind(format!("%{}%", search));
        } else {
            return Err(AppError::Database(sqlx::Error::InvalidArgument(
                "invalid search".to_string(),
            )));
        }
    }
    builder.push(" ORDER BY created_at DESC");
    builder.push(" LIMIT ");
    builder.push_bind(filter.limit as i64);
    builder.push(" OFFSET ");
    builder.push_bind(filter.offset as i64);
    let todos = builder
        .build_query_as::<TodoResponse>()
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
