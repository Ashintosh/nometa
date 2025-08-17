use sqlx::PgPool;

use crate::utils::crypto::{hash_password, verify_password};
use crate::models::user::{AuthUser, CreateUser, User, UserRole, UserSubscription};


const PG_UNIQUE_VIOLATION: &str = "23505";

pub(crate) struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub(crate) async fn get_user_by_id(&self, user_id: i32) -> Result<User, String> {
        let result = sqlx::query!(
            r#"
            SELECT id, email::text, username::text, role as "role: UserRole", subscription as "subscription: UserSubscription", subscription_expires, created_at
            FROM nometa.users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("internal-server-error"))?
        .ok_or_else(|| "invalid-login".to_string())?;

        let user = User {
            id: result.id,
            email: result.email.ok_or("null")?,
            username: result.username.ok_or("null")?,
            role: result.role,
            subscription: result.subscription,
            subscription_expires: result.subscription_expires,
            created_at: result.created_at,
        };

        Ok(user)
    }

    pub(crate) async fn authenticate_user(&self, user: AuthUser) -> Result<i32, String> {
        let result = sqlx::query!(
            r#"
            SELECT id, password FROM nometa.users
            WHERE username = $1
            "#,
            user.username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("internal-server-error"))?
        .ok_or_else(|| "invalid-login".to_string())?;

        if let Err(e) = verify_password(&user.password, &result.password) {
            return Err("invalid-login".to_string());
        }

        Ok(result.id)
    }

    pub(crate) async fn create_user(&self, user: CreateUser) -> Result<(), String> {
        let password_hash = hash_password(&user.password)?;

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
                        Some("users_email_key") => return Err("email-exists".into()),
                        Some("users_username_key") => return Err("username-exists".into()),
                        _ => return Err("duplicate-value".into()),
                    }
                }
            }
            // TODO: Add logging
            return Err(format!("Database error: {}", e));
        }

        Ok(())
    }
}