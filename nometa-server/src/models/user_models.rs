use serde::Deserialize;
use sqlx::types::time::PrimitiveDateTime;


#[derive(Deserialize)]
pub(crate) struct User {
    pub(crate) id: i32,
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) created_at: PrimitiveDateTime,
}

#[derive(Deserialize)]
pub(crate) struct CreateUser {
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Deserialize)]
pub(crate) struct AuthUser {
    pub(crate) username: String,
    pub(crate) password: String,
}