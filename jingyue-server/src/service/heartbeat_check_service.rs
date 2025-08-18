use crate::{application::{BoxError, BoxFuture}, service::BackendService, state::application_state::ApplicationState};

pub struct HeartbeatCheckService;

impl BackendService for HeartbeatCheckService {
    fn start(
        &self,
        state: std::sync::Arc<ApplicationState>,
    ) -> BoxFuture<'static, Result<(), BoxError>> {
        Box::pin(async move {

            
            Ok(())
        })
    }
}
