use anyhow::Result;
use sqlx::{MySql, Pool};

use crate::oauth::Provider;

pub struct User {
    pub pk: i32,
	pub first_name: String,
	pub last_name: String,
	pub provider: String,
	pub provider_user_id: i32,
}

impl User {
    pub async fn find_by_provider_id(pool: &Pool<MySql>, provider: Provider, user_id: i32) -> Result<Self> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users
                WHERE provider = ?
                AND provider_user_id = ?
            "#,
            provider.into::<String>(),
            user_id
        )
            .fetch_one(pool)
            .await?;
        
        Ok(user)
    }
}

