use std::collections::HashMap;

use anyhow::{bail, Result};
use axum::extract::Query;
use axum::http::header;
use axum::response::IntoResponse;
use axum::Extension;
use cookie::time::Duration;
use cookie::Cookie;

use crate::auth::TokenPair;
use crate::models::User;
use crate::error::Error;
use crate::oauth::GithubClient;
use crate::Context;

pub async fn handle_github_callback(
    Extension(ctx): Extension<Context>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    if let Some(code) = query.get("code") {
        let client = GithubClient::new();
        let access_token = client.exchange_code(code).await?;
        let user_id = client.get_user_id(&access_token).await?;

        // Find user or create a new one
        let user_id = match User::find_one("github", user_id) {
            Ok(user) => user.pk,
            None => User::create("github", user_id)?,
        };

        let token_pair    = TokenPair::generate(user_id as u64)?;
        let access_token  = token_pair.access_token();
        let refresh_token = token_pair.refresh_token();

        let access_token_cookie = Cookie::build(("access_token", &access_token))
            .path("/")
            .http_only(true)
            .max_age(Duration::minutes(15))
            .same_site(cookie::SameSite::Strict)
            .secure(true)
            .build();

        let refresh_token_cookie = Cookie::build(("refresh_token", &refresh_token))
            .path("/")
            .http_only(true)
            .max_age(Duration::days(1))
            .same_site(cookie::SameSite::Strict)
            .secure(true)
            .build();

        Ok([
            (header::SET_COOKIE, access_token_cookie.to_string()),
            (header::SET_COOKIE, refresh_token_cookie.to_string()),
        ])
    } else {
        bail!(Error::MissingAuthorizationCode)
    }
}
