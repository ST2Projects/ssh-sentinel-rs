pub mod request;

use sqlx::{FromRow, Type};
use crate::model::UserType::USER;

#[derive(Debug, FromRow)]
pub struct User {
    #[sqlx(default)]
    pub user_id: i64,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub api_key: Option<String>,
    pub enabled: Option<bool>,
    pub user_type: Option<UserType>
}

#[derive(Debug, Type)]
#[sqlx(type_name = "user_type", rename_all = "lowercase")]
pub enum UserType {
    ADMIN,
    USER
}

impl From<Option<UserType>> for UserType{
    fn from(value: Option<UserType>) -> Self {
        match value {
            None => {USER} // Probably want to return an err
            Some(x) => {x}
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            user_id: 0,
            user_name: Some("unknown".to_string()),
            user_email: Some("unknown@domain".to_string()),
            api_key: Some("xxx".to_string()),
            enabled: Option::from(false),
            user_type: Some(USER)
        }
    }
}