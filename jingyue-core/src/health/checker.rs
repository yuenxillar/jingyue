use std::sync::Arc;

use tokio::time::{Duration, interval};

use crate::{
    model::service_instance::ServiceInstance, registry::service_registry::ServiceRegistry,
};

pub struct HealthChecker {
    registry: Arc<ServiceRegistry>,
    interval: Duration,
}

impl HealthChecker {
    pub fn new(registry: Arc<ServiceRegistry>) -> Self {
        Self {
            registry,
            interval: Duration::from_secs(30),
        }
    }

    pub async fn run(&self) {
        let mut interval = interval(self.interval);

        loop {
            interval.tick().await;
            self.check_instances().await;
        }
    }

    async fn check_instances(&self) {
        for service in self.registry.services.iter() {
            for instance in service.value().iter() {
                let healthy = self.check_instance(instance.value()).await;
                if healthy != instance.healthy {
                    self.registry
                        .update_health(instance.id.clone(), healthy)
                        .await;
                }
            }
        }
    }

    async fn check_instance(&self, instance: &ServiceInstance) -> bool {
        // 实现TCP/HTTP健康检查逻辑
        if instance.ephemeral {
            // 临时实例检查心跳超时
            chrono::Utc::now() - instance.last_beat < Duration::from_secs(60)
        } else {
            // 持久实例检查端口连通性
            tokio::net::TcpStream::connect(instance.address)
                .await
                .is_ok()
        }
    }
}
