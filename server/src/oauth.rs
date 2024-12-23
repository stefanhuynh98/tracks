use anyhow::{bail, Context, Result};
use reqwest::Client;
use serde::Deserialize;
use crate::error::Error;

pub enum Provider {
    GitHub,
}

impl std::convert::Into<String> for Provider {
    fn into(self) -> String {
        match self {
            Self::GitHub => "github".into()
        }

    }
}

#[derive(Deserialize)]
pub struct OAuthExchange {
    access_token: String,
}

#[derive(Deserialize)]
pub struct GithubUser {
    id: i64,
}

pub struct GithubClient {
    client: Client,
}

impl GithubClient {
    pub fn new() -> Self {
        let client = Client::new();
         
        Self {
            client,
        }
    }

    pub async fn exchange_code(&self, code: &str) -> Result<String> {
        if cfg!(feature = "integration_test") {
            if code == "INVALID_CODE" {
                bail!(Error::BadAuthorizationCode);
            }

            Ok("access_token".into())
        } else {
            let client_id = std::env::var("GITHUB_CLIENT_ID")
                .context("GITHUB_CLIENT_ID")?;
            let client_secret = std::env::var("GITHUB_CLIENT_SECRET")
                .context("GITHUB_CLIENT_SECRET")?;
            let OAuthExchange { access_token } = self.client
                .post("https://github.com/login/oauth/access_token")
                .query(&[
                    ("code", code),
                    ("client_id", &client_id),
                    ("client_secret", &client_secret),
                ])
                .send()
                .await?
                .json::<OAuthExchange>()
                .await?;
            
            Ok(access_token)
        }
    }

    pub async fn get_user_id(&self, token: &str) -> Result<i64> {
        if cfg!(feature = "integration_test") {
            Ok(1)
        } else {
            let GithubUser { id } = self.client
                .get("https://api.github.com/user")
                .bearer_auth(token)
                .send()
                .await?
                .json()
                .await?;
            
            Ok(id)
        }
    }
}
