pub const USER_TABLE_EXECUTE_SQL: &str = 
"CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,          -- 用户名/账号
    password TEXT NOT NULL,            -- 加盐后的密码哈希值
    salt TEXT NOT NULL,                     -- 随机盐值
    role TEXT NOT NULL DEFAULT 'user',      -- 用户角色，默认'user'
    created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'localtime')),  -- 创建时间
    updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'localtime')),  -- 更新时间
    last_login_at TIMESTAMP,                -- 最后登录时间
    is_active BOOLEAN NOT NULL DEFAULT 1,   -- 账户是否激活
    email TEXT,                             -- 可选邮箱
    phone TEXT                              -- 可选手机号
);

-- 创建索引提高查询效率
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email) WHERE email IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);  -- 新增角色索引";

pub const DEFAULT_ADMIN_EXISTS_SQL: &str = 
"INSERT INTO users (username, password, salt, role, email)
SELECT 
    'jingyue', 
    '3b5d01c5c46193496390998989d8e4e2', 
    'jingyue',
    'admin', 
    'jingyue@example.com'
WHERE NOT EXISTS (SELECT 1 FROM users WHERE username = 'jingyue');";



pub mod user_mapper {
    use crate::model::user::User;
    pub async fn query_user_by_username(executor: &sqlx::Pool<sqlx::Sqlite>, username: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as("SELECT id, username, password, salt, role FROM users WHERE username = ? and is_active = 1")
        .bind(username)
        .fetch_optional(executor)
        .await
    }
}