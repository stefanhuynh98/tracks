use httpmock::MockServer;
use serde::Deserialize;

use crate::error::{Result, ApiError};

#[derive(Deserialize)]
pub struct GithubUser {
    pub id: u64,
}

pub struct GithubClient {
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize)]
pub struct ExchangeSuccess {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

impl GithubClient {
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }

    pub async fn exchange_code(&self, code: &str) -> Result<ExchangeSuccess> {
        if cfg!(feature = "integration_test") {
            return Ok(ExchangeSuccess {
                access_token: "".into(),
                token_type: "".into(),
                scope: "".into(),
            });
        }

        let url = "https://github.com/login/oauth/access_token";

        let request = reqwest::Client::new()
            .post(url)
            .header("Accept", "application/json")
            .query(&[
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("code", &String::from(code)),
            ]);

        let response = request.send().await
            .map_err(|_| ApiError::Internal)?
            .json().await
            .map_err(|_| ApiError::Internal)?;

        Ok(response)
    }

    pub async fn get_user_info(&self, access_token: &str) -> Result<GithubUser> {
        if cfg!(feature = "integration_test") {
            return Ok(GithubUser {
                id: 1,
            });
        }

        let url = "https://api.github.com/user";

        let request = reqwest::Client::new()
            .get(url)
            .header("Accept", "application/json")
            .bearer_auth(access_token);

        let response = request.send().await
            .map_err(|_| ApiError::Internal)?
            .json().await
            .map_err(|_| ApiError::Internal)?;

        Ok(response)
    }
}
