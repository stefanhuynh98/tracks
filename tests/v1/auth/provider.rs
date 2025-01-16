use std::collections::HashMap;

use anyhow::Result;
use axum::http::StatusCode;
use dotenvy_macro::dotenv;
use jsonwebtoken::{
    self as jwt,
    DecodingKey,
    Validation,
    Algorithm,
};

fn derive_uid_from_token(token: &str) -> Result<String> {
    let secret = dotenv!("JWT_SECRET");
    let map: HashMap<String, String> = jwt::decode(
        &token.to_string(),
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?.claims;

    Ok(map.get("uid").unwrap().to_owned())
}

mod any {
    use super::*;

    mod GET {
        use super::*;

        #[tokio::test]
        async fn with_code_and_new_user() -> Result<()> {
            let (server, pool) = crate::common::setup().await?;
            let response = server
                .get("/v1/auth/provider/github")
                .add_query_param("code", "VALID_CODE")
                .await;

            response.assert_status(StatusCode::CREATED);

            // These will panic if one of the cookies does not exist
            let access_token = response .cookie("access_token") .to_string();
            let _ = response.cookie("refresh_token");

            // Test if user derived from access token is present in users table
            let uid = derive_uid_from_token(&access_token)?;
            let rows = sqlx::query!("SELECT * FROM users WHERE pk=? AND oauth_provider=\"github\" LIMIT 1", uid)
                .fetch_all(&pool)
                .await?;

            assert!(rows.len() > 0);

            Ok(())
        }

        #[tokio::test]
        async fn with_code_and_existing_user() -> Result<()> {
            let (server, pool) = crate::common::setup().await?;
            let mut tx = pool.begin().await?;

            // Create a user upfront
            sqlx::query!(
                r#"
                    INSERT INTO users (pk, username, oauth_provider, oauth_id)
                    VALUES (1, "test", "github", 1)
                "#
            )
                .execute(&mut *tx)
                .await?;

            let response = server
                .get("/v1/auth/provider/github")
                .add_query_param("code", "VALID_CODE")
                .await;

            response.assert_status(StatusCode::OK);

            // These will panic if one of the cookies does not exist
            let _ = response.cookie("access_token").to_string();
            let _ = response.cookie("refresh_token");

            Ok(())
        }

        #[tokio::test]
        async fn without_code() -> Result<()> {
            let (server, _) = crate::common::setup().await?;
            let response = server.get("/v1/auth/provider/github").await;

            response.assert_status(StatusCode::UNAUTHORIZED);

            Ok(())
        }

        #[tokio::test]
        async fn with_bad_code() -> Result<()> {
            let (server, _) = crate::common::setup().await?;
            let response = server
                .get("/v1/auth/provider/github")
                .add_query_param("code", "INVALID_CODE")
                .await;

            response.assert_status(StatusCode::UNAUTHORIZED);

            Ok(())
        }
    }
}

mod unsupported {
    use super::*;

    #[tokio::test]
    async fn GET() -> Result<()> {
        let (server, _) = crate::common::setup().await?;
        let response = server
            .get("/v1/auth/provider/unsupported")
            .await;

        response.assert_status(StatusCode::BAD_REQUEST);

        Ok(())
    }
}
