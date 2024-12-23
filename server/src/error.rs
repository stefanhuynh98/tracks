use axum::response::IntoResponse;
use axum::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("User with id {0} not found")]
    UserNotFound(u32),

    #[error("\"{0}\" not set")]
    MissingEnvVar(String),

    #[error("Missing authorizaton code")]
    MissingAuthorizationCode,

    #[error("Bad authorizaton code")]
    BadAuthorizationCode,

    #[error("Internal error")]
    Internal,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::UserNotFound(_)          => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            Self::MissingEnvVar(_)         => Self::Internal.into_response(),
            Self::MissingAuthorizationCode => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
            Self::BadAuthorizationCode     => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
            Self::Internal                 => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
