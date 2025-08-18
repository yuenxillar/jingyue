use jingyue_core::model::login::UserLoginResponse;

use crate::{
    error::api_error::ApiError, execute_statement::user::user_mapper::query_user_by_username,
    util::password_encoder::verify,
};

#[derive(Clone)]
pub struct UserService(sqlx::Pool<sqlx::Sqlite>);

impl UserService {
    pub fn new(db: sqlx::Pool<sqlx::Sqlite>) -> Self {
        UserService(db)
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<UserLoginResponse, ApiError> {
        // 1. 验证用户名和密码
        if username.is_empty() || password.is_empty() {
            return Err("Username or password is empty!".into());
        }

        println!("1");
        // 2. 校验用户与密码长度是否符合
        if username.len() < 5 || password.len() < 5 {
            return Err("Username or password length is too short!".into());
        }

        // 3. 查询用户是否存在
        let user = match query_user_by_username(&self.0, username).await {
            Ok(user) => {
                if let Some(u) = user {
                    u
                } else {
                    return Err("User not found!".into());
                }
            }
            Err(e) => return Err(e.into()),
        };

        // 4. 校验密码是否正确
        if !verify(password, user.salt.as_ref(), &user.password) {
            return Err("Username or password is incorrect!".into());
        }

        // 5. 生成 ascessToken
        let access_token = "".to_string();

        println!("1");
        // 6. 返回登录响应
        let resp = UserLoginResponse {
            access_token,
            token_ttl: 1000 * 60 * 60 * 8,
            global_admin: user.role.as_ref() == "admin",
        };

        Ok(resp)
    }
}
