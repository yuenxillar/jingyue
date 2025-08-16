use dashmap::DashMap;
use std::sync::Arc;

use crate::model::service_instance::ServiceInstance;

type ServiceName = Box<str>;
type InstanceId = String;

pub struct ServiceRegistry {
    // 服务名 -> 实例列表
    services: DashMap<ServiceName, DashMap<InstanceId, Arc<ServiceInstance>>>,
    
    // 反向索引: 实例ID -> 服务名
    instance_index: DashMap<InstanceId, ServiceName>,
    
    // 事件通知通道
    event_tx: tokio::sync::broadcast::Sender<RegistryEvent>,
}


impl ServiceRegistry {
    pub async fn register(&self, instance: ServiceInstance) -> Result<(), RegistryError> {
        let instance = Arc::new(instance);
        let service_name = instance.service_name.clone();
        let instance_id = instance.id.clone();

        // 写入存储
        let entry = self.services
            .entry(service_name.clone())
            .or_insert_with(DashMap::new);
        entry.insert(instance_id.clone(), instance.clone());

        // 更新索引
        self.instance_index.insert(instance_id, service_name.clone());

        // 发送事件
        self.event_tx.send(RegistryEvent::InstanceRegistered {
            service_name,
            instance,
        })?;

        Ok(())
    }
}