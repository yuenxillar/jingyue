pub mod api_response;
use hyper::{body::Bytes, Response};


pub trait IntoResponse {
    
    fn into_response(self) -> Response<http_body_util::Full<Bytes>>;
}