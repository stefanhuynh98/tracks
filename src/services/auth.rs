use std::env::var;

use dotenvy_macro::dotenv;
use serde::{Serialize, Deserialize};
use reqwest::Client;
use jsonwebtoken::{
    self as jwt,
    EncodingKey,
    Header,
    Validation,
    DecodingKey,
    Algorithm,
};

use crate::{Error, Result};

#[derive(Deserialize)]
pub struct GithubUser {
    id: i32,
}

#[derive(Deserialize)]
pub struct OAuthExchange {
    access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: i32,
    pub uid: i32,
}

pub struct Token(String);

impl Token {
    pub fn generate(uid: i32, exp: i32) -> Result<Self> {
        let exp = jwt::get_current_timestamp() as i32 + exp;
        let claims = TokenClaims { uid, exp };
        let secret = dotenv!("JWT_SECRET");
        let token  = jwt::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref())
        ).map_err(|_| Error::Internal)?;

        Ok(Token(token))
    }

    pub fn verify(&self) -> Result<TokenClaims> {
        let claims = jwt::decode(
            &self.0,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::new(Algorithm::HS256),
        ).map_err(|_| Error::InvalidToken)?.claims;

        Ok(claims)
    }
}

pub struct TokenPair {
    access_token: Token,
    refresh_token: Token,
}

impl TokenPair {
    pub fn generate(uid: i32) -> Result<Self> {
        let access_token = Token::generate(uid, 60 * 60 * 15)?;
        let refresh_token = Token::generate(uid, 60 * 60 * 60 * 24)?;

        Ok(Self {
            access_token,
            refresh_token,
        })
    }

    pub fn access_token(&self) -> String {
        self.access_token.0.clone()
    }

    pub fn refresh_token(&self) -> String {
        self.refresh_token.0.clone()
    }
}

#[derive(Debug)]
pub enum OAuthProvider {
    GitHub,
}

impl OAuthProvider {
    pub async fn exchange_code(&self, code: &str) -> Result<String> {
        if cfg!(feature = "integration_test") {
            if code == "INVALID_CODE" {
                return Err(Error::BadAuthorizationCode);
            }

            Ok("access_token".into())
        } else {
            match self {
                Self::GitHub => {
                    let url = "https://github.com/login/oauth/access_token";
                    let client_id = var("GITHUB_CLIENT_ID")?;
                    let client_secret = var("GITHUB_CLIENT_SECRET")?;

                    let OAuthExchange { access_token } = Client::new()
                        .post(url)
                        .query(&[
                            ("code", code),
                            ("client_id", &client_id),
                            ("client_secret", &client_secret),
                        ])
                        .send()
                        .await
                        .map_err(|_| Error::OAuthProviderUnreachable)?
                        .json::<OAuthExchange>()
                        .await
                        .map_err(|_| Error::Internal)?;

                    Ok(access_token)
                },
            }
        }
    }

    pub async fn get_user_id(&self, token: &str) -> Result<i32> {
        if cfg!(feature = "integration_test") {
            Ok(1)
        } else {
            match self {
                Self::GitHub => {
                    let url = "https://api.github.com/user";
                    let GithubUser { id } = Client::new()
                        .get(url)
                        .bearer_auth(token)
                        .send()
                        .await
                        .map_err(|_| Error::OAuthProviderUnreachable)?
                        .json()
                        .await
                        .map_err(|_| Error::Internal)?;

                    Ok(id)
                }
            }
        }
    }
}

impl TryFrom<String> for OAuthProvider {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "GitHub" => Ok(Self::GitHub),
            "Github" => Ok(Self::GitHub),
            "github" => Ok(Self::GitHub),
            _ => Err(Error::OAuthProviderUnsupported),
        }
    }
}

impl ToString for OAuthProvider {
    fn to_string(&self) -> String {
        match self {
            Self::GitHub => "github".into(),
        }
    }
}
