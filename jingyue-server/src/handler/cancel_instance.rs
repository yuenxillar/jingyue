use std::sync::Arc;

use http_body_util::BodyExt;
use hyper::Response;
use jingyue_core::model::instance::CancelInstanceRequestBody;

use crate::{
    error::api_error::ApiError,
    response::{IntoResponse, api_response::ApiResponse},
    state::application_state::ApplicationState,
};

use super::Request;

pub(crate) async fn handle_cancel_instance(
    req: Request,
    state: Arc<ApplicationState>,
) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, ApiError> {
    let body = req.collect().await?.to_bytes();

    let data: CancelInstanceRequestBody = serde_json::from_slice(&body)?;
    Ok(ApiResponse::success("ok".to_string()).into_response())
}
