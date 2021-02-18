use super::schema::cards;
use super::schema::users;
use serde::Serialize;
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub pass: &'a str,
}

#[derive(Identifiable, Queryable, Serialize, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub pass: String,
}

#[derive(Insertable)]
#[table_name = "cards"]
pub struct NewLoyalty<'a> {
    pub name: &'a str,
    pub color: Option<&'a str>,
    pub code: &'a str,
    pub user_id: i32,
}

#[derive(Identifiable, Serialize, Queryable)]
#[table_name = "cards"]
pub struct Loyalty {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
    pub code: String,
    pub user_id: i32,
}

pub struct LoyaltyUpdate<'a> {
    pub name: &'a str,
    pub color: Option<&'a str>,
    pub code: &'a str,
}
