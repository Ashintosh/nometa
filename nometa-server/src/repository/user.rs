use sqlx::PgPool;

use crate::utils::crypto::{hash_password, verify_password};
use crate::models::user::{AuthUser, CreateUser, User};


const PG_UNIQUE_VIOLATION: &str = "23505";

pub(crate) struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub(crate) async fn authenticate_user(&self, user: AuthUser) -> Result<User, String> {
        let result = sqlx::query!(
            r#"
            SELECT id, email::text, username::text, password, created_at
            FROM nometa.users
            WHERE username = $1
            "#,
            user.username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Invalid username or password".to_string())?;

        let stored_hash = sqlx::query_scalar!(
            "SELECT password FROM nometa.users WHERE username = $1",
            user.username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e.to_string()))?
        .ok_or_else(|| { "User not found".to_string()})?;

        if ! verify_password(&user.password, &stored_hash) {
            return Err("Invalid username or password".into());
        }

        let authed_user = User {
            id: result.id,
            email: result.email.ok_or_else(|| "Empty email from database".to_string())?,
            username: result.username.ok_or_else(|| "Empty username from database".to_string())?,
            created_at: result.created_at,
        };

        Ok(authed_user)
    }

    pub(crate) async fn create_user(&self, user: CreateUser) -> Result<(), String> {
        let password_hash = hash_password(&user.password);

        let result = sqlx::query!(
            r#"
            INSERT INTO nometa.users (email, username, password) 
            VALUES ($1::text, $2::text, $3)
            "#,
            user.email,
            user.username,
            password_hash,
        )
        .execute(&self.pool)
        .await;

        if let Err(e) = result {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code().as_deref() == Some(PG_UNIQUE_VIOLATION) {
                    match db_err.constraint() {
                        Some("users_email_key") => return Err("Email already exists".into()),
                        Some("users_username_key") => return Err("Username already exists".into()),
                        _ => return Err("Duplicate value".into()),
                    }
                }
            }
            return Err(format!("Database error: {}", e));
        }

        Ok(())
    }
}