use std::fmt::Display;

use http_body_util::Full;
use hyper::{Response, StatusCode};

use crate::response::IntoResponse;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApiError {
    // 400 Bad Request
    BadRequest(String),

    // 401 Unauthorized
    Unauthorized,

    // 500 Internal Server Error
    InternalServerError,

    // 404 Not Found
    NotFound,

    Customized(String),
}

impl ApiError {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Customized(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Self::BadRequest(msg) => msg,
            Self::Unauthorized => "Unauthorized",
            Self::InternalServerError => "Internal Server Error",
            Self::NotFound => "Not Found",
            Self::Customized(msg) => msg,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> hyper::Response<http_body_util::Full<hyper::body::Bytes>> {
        Response::builder()
            .status(self.status())
            .body(Full::new(self.message().to_string().into()))
            .unwrap()
    }
}


impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            Self::Unauthorized => write!(f, "Unauthorized"),
            Self::InternalServerError => write!(f, "Internal Server Error"),
            Self::NotFound => write!(f, "Not Found"),
            Self::Customized(msg) => write!(f, "Customized: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {
}

impl Into<ApiError> for &'static str  {
    fn into(self) -> ApiError {
        ApiError::Customized(self.to_string())
    }
}

impl Into<ApiError> for &dyn std::error::Error {
    fn into(self) -> ApiError {
        ApiError::Customized(self.to_string())
    }
}

impl From<hyper::Error> for ApiError {
    fn from(error: hyper::Error) -> Self {
        ApiError::Customized(error.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        ApiError::Customized(error.to_string())
    }
}


impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        ApiError::Customized(error.to_string())
    }
}