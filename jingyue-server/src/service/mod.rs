pub mod heartbeat_check_service;
pub mod instance_service;
pub mod user_service;



use std::sync::Arc;

use crate::{
    application::{BoxError, BoxFuture},
    state::application_state::ApplicationState,
};

pub trait BackendService: Send + Sync {
    fn start(&self, state: Arc<ApplicationState>) -> BoxFuture<'static, Result<(), BoxError>>;
}



pub trait Service: Send + Sync {
    fn ready(&self, state: Arc<ApplicationState>) -> BoxFuture<'static, Result<(), BoxError>>;
}