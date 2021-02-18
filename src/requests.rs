use serde::Deserialize;
use validator::Validate;

use crate::db::models::NewUser;
#[derive(Debug, Deserialize, Validate)]
pub struct UserSignup {
    #[validate(email)]
    pub email: String,
    pub name: String,
    pub pass: String,
}

#[derive(Deserialize)]
pub struct UserSignIn {
    pub email: String,
    pub pass: String,
}
