use std::sync::Arc;
use std::collections::HashMap;

use axum::http::header;
use axum::extract::{State, Path, Query};
use axum::response::{AppendHeaders, IntoResponse};
use cookie::time::Duration;
use cookie::Cookie;
use reqwest::StatusCode;

use crate::services::auth::{OAuthProvider, TokenPair};
use crate::services::user::{
    create_user_unregistered,
    find_user_by_provider,
};
use crate::{AppState, Error, Result};

pub async fn handle_oauth_callback(
    State(state): State<Arc<AppState>>,
    Path(provider): Path<String>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    let provider: OAuthProvider = provider.try_into()?;

    if let Some(code) = query.get("code") {
        let access_token = provider.exchange_code(code).await?;
        let provider_user_id = provider.get_user_id(&access_token).await?;

        let user_id = match find_user_by_provider(&state.pool, OAuthProvider::GitHub, provider_user_id).await? {
            Some(user) => user.pk,
            None => create_user_unregistered(&state.pool, OAuthProvider::GitHub, provider_user_id).await?,
        };

        let token_pair    = TokenPair::generate(user_id)?;
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

        let headers = AppendHeaders([
            (header::SET_COOKIE, access_token_cookie.to_string()),
            (header::SET_COOKIE, refresh_token_cookie.to_string()),
        ]);

        Ok((StatusCode::CREATED, headers))
    } else {
        Err(Error::NoAuthorizationCode)
    }
}
