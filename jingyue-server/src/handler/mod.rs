pub mod get_instance_list;
pub mod publish_config;
pub mod get_config;
pub mod cancel_instance;
pub mod register_instance;
pub mod request_handler;
pub mod user_login;

pub(super) type Request = hyper::Request<hyper::body::Incoming>;