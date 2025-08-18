use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize)]
pub struct UserLoginRequestBody {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginResponse {
    pub access_token: String,
    pub token_ttl: u32,
    pub global_admin: bool,
}