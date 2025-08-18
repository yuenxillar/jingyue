use crate::execute_statement::user::{DEFAULT_ADMIN_EXISTS_SQL, USER_TABLE_EXECUTE_SQL};

pub mod user;

pub(crate) fn all_init_execute_sql() -> Vec<&'static str> {
    vec![USER_TABLE_EXECUTE_SQL, DEFAULT_ADMIN_EXISTS_SQL]
}
