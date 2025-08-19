use std::{convert::Infallible, sync::Arc};

use http_body_util::Full;
use hyper::{Method, Response, StatusCode, body::Bytes};

use crate::{
    error::api_error::ApiError, response::IntoResponse, state::application_state::ApplicationState,
};

use super::{
    Request, cancel_instance::handle_cancel_instance, get_config::handle_get_config,
    get_instance_list::handle_get_instance_list, publish_config::handle_publish_config,
    register_instance::handle_register_instance, user_login::handle_user_login,
};

pub(crate) async fn handle_request(
    req: Request,
    state: Arc<ApplicationState>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    
    let resp =match (req.method(), req.uri().path()) {
        (&Method::POST, "/jingyue/v1/client/ns/instance") => {
            handle_register_instance(req, state).await
        }
        (&Method::DELETE, "/jingyue/v1/client/ns/instance") => {
            handle_cancel_instance(req, state).await
        }
        (&Method::GET, "/jingyue/v1/client/ns/instance/list") => {
            handle_get_instance_list(req, state).await
        }

        (&Method::POST, "/jingyue/v1/client/cs/configs") => handle_publish_config(req, state).await,
        (&Method::GET, "/jingyue/v1/client/cs/configs") => handle_get_config(req, state).await,

        (&Method::POST, "/jingyue/v1/auth/user/login") => handle_user_login(req, state).await,

        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("Not Found")))
            .unwrap()),
    }.map_err(ApiError::into_response);
    Ok(
        resp.unwrap_or_else(|e| e)
    )
}
