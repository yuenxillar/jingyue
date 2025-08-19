use jingyue_core::model::instance::{CancelInstanceRequestBody, InstanceRequestBody};
use tracing::{error, info};

use crate::model::instance::Instance;

#[derive(Clone)]
pub struct InstanceService {
    pub(crate) instances: dashmap::DashMap<String, Instance>,
}

impl InstanceService {
    pub async fn register(&self, instance: InstanceRequestBody) -> Result<(), ()> {
        let is_heart_beat = instance.heart_beat;
        let instance: Instance = instance.into();

        match is_heart_beat {
            true => {
                let key = self.generate_key(
                    &instance.info.service_name,
                    &instance.info.ip,
                    instance.info.port,
                );
                if let Some(value) = self.instances.get(&key) {
                    value.record().await;
                };
                Ok(())
            }
            false => {
                let key = self.generate_key(
                    &instance.info.service_name,
                    &instance.info.ip,
                    instance.info.port,
                );
                match self.instances.insert(key, instance) {
                    Some(_) => {
                        error!("Instance already exists");
                        Err(())
                    }
                    None => Ok(()),
                }
            }
        }
    }

    pub fn cancel_instance(&self, body: CancelInstanceRequestBody) -> Result<bool, ()> {
        let key = self.generate_key(&body.service_name, &body.ip, body.port);
        match self.instances.remove(&key) {
            Some(_) => Ok(true),
            None => Err(()),
        }
    }

    fn generate_key(&self, service_name: &str, ip: &str, port: u32) -> String {
        format!("{}:{}:{}", service_name, ip, port)
    }
}
