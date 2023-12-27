use crate::database::postgres::Db;
use crate::error::Result;
use crate::models::user::{User, UserConditions, UserDTO, UserId, UserList};
use anyhow::Context;
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
    async fn add(&self, user_data: &UserDTO) -> Result<UserId>;
    async fn find_by_email(&self, email: &String) -> Result<User>;
    async fn find_all(&self, conditions: &UserConditions) -> Result<UserList>;
}

#[async_trait]
impl UserRepo for UserRepoImpl {
    async fn add(&self, user_data: &UserDTO) -> Result<UserId> {
        let row = sqlx::query_as::<_, UserId>(
            r#"
            insert into users (email, password_hash)
            values ($1, $2)
            returning id
            "#,
        )
        .bind(&user_data.email)
        .bind(&user_data.password)
        .fetch_one(&*self.pool)
        .await
        .context("DB ERROR (create user)")?;
        Ok(row)
    }

    async fn find_by_email(&self, email: &String) -> Result<User> {
        let result = sqlx::query_as::<_, User>("select first from users where email LIKE $1")
            .bind(format!("%{}%", email))
            .fetch_one(&*self.pool)
            .await
            .context("DB ERROR (find all users)")?;
        Ok(result)
    }

    async fn find_all(&self, conditions: &UserConditions) -> Result<UserList> {
        let mut query = sqlx::query_as::<_, User>("select * from users");
        if let Some(email) = &conditions.email {
            query = sqlx::query_as::<_, User>("select * from users where email LIKE $1")
                .bind(format!("%{}%", email))
        }
        let result = query
            .fetch_all(&*self.pool)
            .await
            .context("DB ERROR (find all users)")?;
        Ok(result)
    }
}
