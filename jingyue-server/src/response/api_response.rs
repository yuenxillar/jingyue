use http_body_util::Full;
use hyper::{Response, StatusCode, body::Bytes};
use serde::{Deserialize, Serialize};

use super::IntoResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> where T: serde::Serialize {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            code: -1,
            message,
            data: None,
        }
    }
}



impl<T> IntoResponse for ApiResponse<T>
where
    Self: serde::ser::Serialize,
    T: serde::ser::Serialize,
{
    fn into_response(self) -> hyper::Response<http_body_util::Full<Bytes>> {
        Response::builder()
            .status(StatusCode::OK)
            .body(Full::new(serde_json::to_string(&self).unwrap().into()))
            .unwrap()
    }
}
