use std::time::Duration;

use tracing::info;

use crate::{
    application::{BoxError, BoxFuture},
    service::BackendService,
    state::application_state::ApplicationState,
};

#[derive(Default)]
pub struct HeartbeatCheckService;

impl BackendService for HeartbeatCheckService {
    fn start(
        &self,
        state: std::sync::Arc<ApplicationState>,
    ) -> BoxFuture<'static, Result<(), BoxError>> {
        Box::pin(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
            loop {
                interval.tick().await;

                let mut keys = Vec::new();
                for item in state.instance_service.instances.iter() {
                    let now = tokio::time::Instant::now();
                    if !item.is_healthy().await && (now - item.created_at >= Duration::from_secs(5))
                    {
                        keys.push(item.key().clone());
                    }
                }
                keys.iter().for_each(|s| {
                    state.instance_service.instances.remove(s);
                    info!("Instance [{}] is removed due to heartbeat failure", s);
                });
            }
        })
    }
}
