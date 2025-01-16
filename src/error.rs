use std::env::VarError;

use axum::http::StatusCode;
use axum::response::IntoResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Internal Error")]
    Internal,

    #[error("OAuth provider is not supported")]
    OAuthProviderUnsupported,

    #[error("OAuth provider unreachable")]
    OAuthProviderUnreachable,

    #[error("Received invalid authorization code from OAuth provider")]
    BadAuthorizationCode,

    #[error("Missing authorization code from OAuth provider")]
    NoAuthorizationCode,

    // @TODO: Currently unable to provide context (the variable in question).
    #[error("Some environment variable was not set")]
    EnvVarNotSet,

    // @TODO: Currently unable to provide context (the variable in question).
    #[error("Environment variable is invalid")]
    EnvVarInvalid,

    #[error("Invalid token")]
    InvalidToken,
}

// When environment variables are not set
impl From<VarError> for Error {
    fn from(value: VarError) -> Self {
        match value {
            VarError::NotPresent => Error::EnvVarNotSet,
            VarError::NotUnicode(_) => Error::EnvVarInvalid,
        }
    }
}

// When database errors occur
impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            _ => Error::Internal,
        }
    }
}

// Make this error type response compatible
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        // Use this opportunity to report internal errors
        // @TODO: questioning if this is the right way to do this,
        // but for now, any logging is better than nothing.
        match self {
            Self::EnvVarNotSet         => tracing::error!("ERR_ENV_VAR_NOT_SET"),
            Self::EnvVarInvalid        => tracing::error!("ERR_ENV_VAR_INVALID"),
            Self::BadAuthorizationCode => tracing::warn!("ERR_AUTH_CODE_INVALID"),
            _ => {}
        };

        match self {
            Self::OAuthProviderUnsupported => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Self::OAuthProviderUnreachable => (StatusCode::BAD_GATEWAY, self.to_string()).into_response(),
            Self::BadAuthorizationCode     => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
            Self::NoAuthorizationCode      => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
            _                              => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
