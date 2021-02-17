use serde::Deserialize;
use validator::Validate;

use crate::db::models::NewUser;
#[derive(Debug, Deserialize, Validate)]
pub struct UserSignup {
    pub email: String,
    pub name: String,
    pub pass: String,
}

impl<'a> From<&'a UserSignup> for NewUser<'a> {
    fn from(o: &'a UserSignup) -> Self {
        NewUser {
            email: &o.email,
            name: &o.name,
            pass: &o.pass,
        }
    }
}
