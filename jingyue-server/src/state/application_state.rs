use crate::{
    ApplicationArgs, config::application_config::ApplicationConfig,
    service::user_service::UserService,
};

#[derive(Clone)]
pub struct ApplicationState {
    pub db: sqlx::Pool<sqlx::Sqlite>,
    pub config: ApplicationConfig,
    pub args: ApplicationArgs,

    pub user_service: UserService,
}
