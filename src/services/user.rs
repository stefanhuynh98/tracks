use sqlx::{FromRow, MySqlPool};
use sqlx::types::chrono::NaiveDateTime;

use crate::util::random_username;
use crate::Result;

use super::auth::OAuthProvider;

#[derive(Debug, FromRow)]
pub struct User {
    pub pk: i32,
    pub username: String,
    pub oauth_id: i32,
    pub oauth_provider: String,
    pub registered_since: Option<NaiveDateTime>,
}

// @TODO: Add registered_since field
pub async fn create_user(pool: &MySqlPool, username: &str, oauth_provider: OAuthProvider, provider_user_id: i32) -> Result<i32> {
    let user_id = sqlx::query!(
        r#"
            INSERT INTO users (username, oauth_provider, oauth_id)
            VALUES (?, ?, ?);
        "#,
        username,
        oauth_provider.to_string(),
        provider_user_id,
    )
        .execute(pool)
        .await?
        .last_insert_id();

    Ok(user_id as i32)
}

pub async fn create_user_unregistered(pool: &MySqlPool, oauth_provider: OAuthProvider, provider_user_id: i32) -> Result<i32> {
    let username = random_username();
    let user_id = sqlx::query!(
        r#"
            INSERT INTO users (username, oauth_provider, oauth_id)
            VALUES (?, ?, ?);
        "#,
        username,
        oauth_provider.to_string(),
        provider_user_id,
    )
        .execute(pool)
        .await?
        .last_insert_id();

    Ok(user_id as i32)
}

pub async fn find_user_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT * FROM users WHERE username=?
        "#,
        username
    )
        .fetch_one(pool)
        .await?;

    Ok(Some(user))
}

pub async fn find_user_by_provider(pool: &MySqlPool, oauth_provider: OAuthProvider, provider_user_id: i32) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT * FROM users WHERE oauth_id=? AND oauth_provider=?
        "#,
        provider_user_id,
        oauth_provider.to_string(),
    )
        .fetch_one(pool)
        .await;

    match user {
        Err(sqlx::Error::RowNotFound) => Ok(None),
        Err(e) => Err(e.into()),
        Ok(user) => Ok(Some(user))
    }
}
