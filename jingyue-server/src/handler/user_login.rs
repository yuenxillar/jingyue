use std::sync::Arc;

use http_body_util::BodyExt;
use hyper::Response;
use jingyue_core::model::login::UserLoginRequestBody;

use crate::{
    error::api_error::ApiError,
    response::{IntoResponse, api_response::ApiResponse},
    state::application_state::ApplicationState,
};

use super::Request;

pub(crate) async fn handle_user_login(
    req: Request,
    state: Arc<ApplicationState>,
) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, ApiError> {
    let body = req.collect().await?.to_bytes();

    let req_body: UserLoginRequestBody = serde_json::from_slice(&body)?;

    let resp = state
        .user_service
        .login(&req_body.username, &req_body.password)
        .await?;

    Ok(ApiResponse::success(resp).into_response())
}
