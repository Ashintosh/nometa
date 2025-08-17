use serde::{Deserialize, Serialize};
use sqlx::Type;
use sqlx::types::time::PrimitiveDateTime;


#[derive(Debug, Deserialize, Serialize, Type)]
#[sqlx(type_name = "nm_user_role")]
#[sqlx(rename_all = "lowercase")]
pub(crate) enum UserRole {
    Banned,
    User,
    Admin,
    Owner,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[sqlx(type_name = "nm_user_subscription")]
#[sqlx(rename_all = "lowercase")]
pub(crate) enum UserSubscription {
    Free,
    Premium,
    Vip,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct User {
    pub(crate) id: i32,
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) role: UserRole,
    pub(crate) subscription: UserSubscription,
    pub(crate) subscription_expires: Option<PrimitiveDateTime>,
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