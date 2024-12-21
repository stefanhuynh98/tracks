use axum::http::{HeaderMap, HeaderValue, header};
use axum::response::IntoResponse;
use axum::extract::Query;
use serde::Deserialize;

use crate::auth::oauth::{ExchangeSuccess, GithubClient};
use crate::error::{ApiError, Result};

#[derive(Deserialize)]
pub struct Code {
    code: String,
}

pub async fn handle_github_callback<'a>(Query(Code { code }): Query<Code>) -> Result<impl IntoResponse> {
    let client = GithubClient::new(
        "Ov23lihidsMbCWLrxdhv", 
        "d6864f8c448de57490a3037ecd41ebb79372adae"
    );
    let ExchangeSuccess { access_token, .. } = client.exchange_code(&code).await?;
    let user = client.get_user_info(&access_token).await?;

    // Generate new token pair
    let access_token: String = todo!();
    let refresh_token: String = todo!();

    let headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(
            &format!("access_token={}; Path=/; HttpOnly; MaxAge={}; SameSite=Strict", access_token, 60 * 60 * 15)
        ).map_err(|_| ApiError::Internal)?
    );
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(
            &format!("refresh_token={}; Path=/; HttpOnly; MaxAge={}; SameSite=Strict", refresh_token, 60 * 60 * 60 * 24)
        ).map_err(|_| ApiError::Internal)?
    );

    Ok((headers))
}
