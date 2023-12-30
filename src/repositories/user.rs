use crate::database::postgres::Db;
use crate::error::{AppError, Result};
use crate::models::user::{User, UserId, UserInput};
use async_trait::async_trait;

#[derive(Clone)]
pub struct UserRepoImpl {
    pool: Db,
}
impl UserRepoImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool: pool }
    }
}

#[async_trait]
pub trait UserRepo {
    async fn add(&self, input: &UserInput) -> Result<UserId>;
    async fn find_by_email(&self, email: &String) -> Result<User>;
}

#[async_trait]
impl UserRepo for UserRepoImpl {
    async fn add(&self, input: &UserInput) -> Result<UserId> {
        let result = sqlx::query_as::<_, UserId>(
            r#"
            INSERT INTO users (email, password_hash)
            VALUES ($1, $2)
            RETURNING id
            "#,
        )
        .bind(&input.email)
        .bind(&input.password)
        .fetch_one(&*self.pool)
        .await;

        match result {
            Ok(user_id) => Ok(user_id),
            Err(e) => {
                tracing::error!("Database Error: {:?}", e);
                Err(AppError::Conflict("Email already exists"))
            }
        }
    }

    async fn find_by_email(&self, email: &String) -> Result<User> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email=$1")
            .bind(email)
            .fetch_one(&*self.pool)
            .await;

        match result {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::error!("Database Error: {:?}", e);
                Err(AppError::Unauthorized("Invalid email or password"))
            }
        }
    }
}
