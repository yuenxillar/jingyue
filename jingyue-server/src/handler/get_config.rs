use std::{collections::HashMap, sync::Arc};

use http_body_util::BodyExt;
use hyper::Response;
use jingyue_core::model::config::{GetConfigParams, GetConfigResponse};

use crate::{
    error::api_error::ApiError,
    response::{api_response::ApiResponse, IntoResponse},
    state::application_state::ApplicationState,
};

use super::Request;

pub(crate) async fn handle_get_config(
    req: Request,
    state: Arc<ApplicationState>,
) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, ApiError> {
    let b = req.collect().await?.to_bytes();
    let params = form_urlencoded::parse(b.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();

    let config: GetConfigParams = GetConfigParams::from(params);

    Ok(ApiResponse::success(GetConfigResponse::default()).into_response())
}
