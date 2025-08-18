use std::{collections::HashMap, sync::Arc};

use http_body_util::BodyExt;
use hyper::Response;
use jingyue_core::model::instance::{GetServiceInstanceListParams, InstanceItemsResponse};

use crate::{
    error::api_error::ApiError,
    response::{IntoResponse, api_response::ApiResponse},
    state::application_state::ApplicationState,
};

use super::Request;

pub(crate) async fn handle_get_instance_list(
    req: Request,
    state: Arc<ApplicationState>,
) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, ApiError> {
    let body = req.collect().await?.to_bytes();

    let params = form_urlencoded::parse(body.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();

    let params: GetServiceInstanceListParams = GetServiceInstanceListParams::from(params);

    Ok(ApiResponse::success(Vec::<InstanceItemsResponse>::new()).into_response())
}
