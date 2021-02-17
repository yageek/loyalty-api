#[macro_use]
extern crate diesel;
mod db;
mod requests;

use db::models::NewUser;
use diesel::RunQueryDsl;
use requests::UserSignup;
use rocket::{
    http::{ContentType, Status},
    launch, post,
    response::Responder,
    routes, Response,
};

use rocket_contrib::{database, json::Json};
use thiserror::Error;
use validator::{Validate, ValidationError};

#[derive(Debug, Error)]
enum SignupError {
    #[error("error during sign in")]
    ValidationError,
    #[error("query error")]
    DieselError(#[from] diesel::result::Error),
}

impl<'a> Responder<'a, 'static> for SignupError {
    fn respond_to(self, request: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .header(ContentType::JSON)
            .status(Status::BadRequest)
            .ok()
    }
}

#[database("loyalty_db")]
struct LoyaltyDbConn(diesel::SqliteConnection);

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(LoyaltyDbConn::fairing())
        .mount("/", routes![signup])
}

#[post("/signup", format = "json", data = "<body>")]
async fn signup(db: LoyaltyDbConn, body: Json<UserSignup>) -> Result<(), SignupError> {
    if body.validate().is_err() {
        Err(SignupError::ValidationError)
    } else {
        db.run(move |c| {
            let new_value = NewUser {
                email: &body.0.email,
                name: &body.0.name,
                pass: &body.0.pass,
            };

            let elem = diesel::insert_into(db::schema::users::table)
                .values(&new_value)
                .execute(c)?;
            println!("Elemeent: {}", elem);
            Ok(())
        })
        .await
    }
}
