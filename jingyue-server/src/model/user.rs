use sqlx::{prelude::FromRow, types::chrono::NaiveDateTime};


#[derive(Clone, Debug, FromRow)]
pub struct User {
    pub id: u32,
    pub username: String,   
    pub password: String,
    pub salt: Box<str>,
    pub role: Box<str>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub last_login_at: Option<NaiveDateTime>,
    pub is_active: bool,
    pub email: Option<String>,
    pub phone: Option<String>,
}