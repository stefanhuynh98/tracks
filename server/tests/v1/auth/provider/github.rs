use std::collections::HashMap;

use anyhow::{Context, Result};
use axum::http::StatusCode;
use jsonwebtoken::{
    self as jwt,
    DecodingKey,
    Validation,
    Algorithm,
};

fn derive_uid_from_token(token: &str) -> Result<String> {
    let secret = std::env::var("JWT_SECRET").context("JWT_SECRET not set")?;
    let map: HashMap<String, String> = jwt::decode(
        &token.to_string(),
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?.claims;

    Ok(map.get("uid").unwrap().to_owned())
}

#[allow(non_snake_case)]
#[tokio::test]
async fn GET_with_code_and_new_user() -> Result<()> {
    let (server, pool) = crate::util::setup().await?;
    let response = server
        .get("/v1/auth/provider/github")
        .add_query_param("code", "VALID_CODE")
        .await;
    
    response.assert_status(StatusCode::CREATED);

    // This will panic if one of the cookies does not exist
    let access_token = response.cookie("access_token").to_string();
    let _ = response.cookie("refresh_token");

    // Test if user derived from access token is added to users table
    let uid = derive_uid_from_token(&access_token)?;
    let rows = sqlx::query!("SELECT * FROM users WHERE pk=? AND provider=\"github\" LIMIT 1", uid)
        .fetch_all(&pool)
        .await?;

    assert!(rows.len() > 0);

    Ok(())
}

#[allow(non_snake_case)]
#[tokio::test]
async fn GET_with_code_and_existing_user() -> Result<()> {
    let (server, pool) = crate::util::setup().await?;
    let mut tx = pool.begin().await?;

    // Create a user upfront
    sqlx::query!(
        r#"
        INSERT INTO users (pk, first_name, last_name, provider, provider_user_id)
        VALUES (1, "test", "user", "github", 1)
        "#
    )
        .execute(&mut *tx)
        .await?;

    let response = server
        .get("/v1/auth/provider/github")
        .add_query_param("code", "VALID_CODE")
        .await;

    response.assert_status(StatusCode::OK);

    // This will panic if one of the cookies does not exist
    let _ = response.cookie("access_token").to_string();
    let _ = response.cookie("refresh_token");

    Ok(())
}

#[allow(non_snake_case)]
#[tokio::test]
async fn GET_without_code() -> Result<()> {
    let (server, _) = crate::util::setup().await?;
    let response = server.get("/v1/auth/provider/github").await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    Ok(())
}

#[allow(non_snake_case)]
#[tokio::test]
async fn GET_with_bad_code() -> Result<()> {
    let (server, _) = crate::util::setup().await?;
    let response = server
        .get("/v1/auth/provider/github")
        .add_query_param("code", "INVALID_CODE")
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    Ok(())
}
