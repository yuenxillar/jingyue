


#[derive(Clone)]
pub struct ApplicationState {
    pub db: sqlx::Pool<sqlx::Sqlite>
}