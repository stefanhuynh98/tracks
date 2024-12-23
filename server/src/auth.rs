use anyhow::Result;
use jsonwebtoken::{
    self as jwt,
    EncodingKey,
    Header,
    Validation,
    DecodingKey,
    Algorithm,
};
use serde::{Serialize, Deserialize};

use crate::error::Error;

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: u64,
    pub uid: u64,
}

pub struct Token(String);

impl Token {
    pub fn generate(uid: u64, exp: u64) -> Result<Self> {
        let exp = jwt::get_current_timestamp() + exp;
        let claims = TokenClaims { uid, exp };
        let secret = std::env::var("JWT_SECRET").map_err(|_| {
            Error::MissingEnvVar("JWT_SECRET".into())
        })?;
        let token  = jwt::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref())
        )?;

        Ok(Token(token))
    }

    pub fn verify(&self) -> Result<TokenClaims> {
        let claims = jwt::decode(
            &self.0,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::new(Algorithm::HS256),
        )?.claims;

        Ok(claims)
    }
}

pub struct TokenPair {
    access_token: Token,
    refresh_token: Token,
}

impl TokenPair {
    pub fn generate(uid: u64) -> Result<Self> {
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
