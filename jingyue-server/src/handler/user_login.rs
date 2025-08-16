use std::sync::Arc;

use http_body_util::BodyExt;
use hyper::Response;

use crate::{
    error::api_error::ApiError,
    model::login::{UserLoginBody, UserLoginResponse},
    response::{IntoResponse, api_response::ApiResponse},
    state::application_state::ApplicationState,
};

use super::Request;

pub(crate) async fn handle_user_login(
    req: Request,
    state: Arc<ApplicationState>,
) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, ApiError> {
    let body = req.collect().await?.to_bytes();

    let body: UserLoginBody = serde_json::from_slice(&body)?;

    let resp = UserLoginResponse {
        access_token: String::from("access_token"),
        token_ttl: 1000,
        global_admin: false,
    };

    Ok(ApiResponse::success(resp).into_response())
}
