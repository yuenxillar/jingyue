use crate::{
    ApplicationArgs,
    config::application_config::ApplicationConfig,
    service::{instance_service::InstanceService, user_service::UserService},
};

#[derive(Clone)]
pub struct ApplicationState {
    pub db: sqlx::Pool<sqlx::Sqlite>,
    pub config: ApplicationConfig,
    pub args: ApplicationArgs,

    pub instance_service: InstanceService,
    pub user_service: UserService,
}
