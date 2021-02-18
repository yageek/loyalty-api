use serde::{Deserialize, Serialize};
use validator::Validate;

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

#[derive(Deserialize)]
pub struct AddLoyalty {
    pub name: String,
    pub color: Option<String>,
    pub code: String,
}

#[derive(Serialize)]
pub struct AddLoyaltyResponse {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub code: String,
}
