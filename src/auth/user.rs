use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct User {
    id: Uuid,
    email: String,
    password_hash: String,
    created_at: NaiveDateTime,
}

