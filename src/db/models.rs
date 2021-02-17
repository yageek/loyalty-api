use super::schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub pass: &'a str,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub pass: String,
}
