use super::schema::users;
use serde::Serialize;
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub pass: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct UserFetch {
    pub id: Option<i32>,
    pub email: String,
    pub name: String,
    pub pass: String,
}
