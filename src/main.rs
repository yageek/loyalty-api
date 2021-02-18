#[macro_use]
extern crate diesel;
mod db;
mod requests;
use diesel::{prelude::*, result::DatabaseErrorKind};

use db::models::NewUser;
use diesel::RunQueryDsl;
use requests::{UserSignIn, UserSignup};

use rocket::{
    get,
    http::{ContentType, RawStr, Status},
    launch, post,
    response::{status, Responder},
    routes, Response,
};
use rocket::{http::Cookie, outcome::IntoOutcome};
use rocket::{http::CookieJar, request::FromRequest};
use rocket_contrib::{database, json::Json};
use thiserror::Error;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Error)]
enum APIError {
    #[error("error during sign in")]
    SignError(#[from] ValidationErrors),
    #[error("query error")]
    DieselError(#[from] diesel::result::Error),
}

impl<'a> Responder<'a, 'static> for APIError {
    fn respond_to(self, request: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut resp = Response::build();

        let status = match self {
            APIError::SignError(..) => Status::BadRequest,
            APIError::DieselError(ref e) => match e {
                diesel::result::Error::DatabaseError(kind, ..)
                    if matches!(kind, DatabaseErrorKind::UniqueViolation) =>
                {
                    Status::BadRequest
                }
                _ => Status::InternalServerError,
            },
            _ => Status::InternalServerError,
        };

        resp.status(status).ok()
    }
}

#[database("loyalty_db")]
struct LoyaltyDbConn(diesel::SqliteConnection);

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(LoyaltyDbConn::fairing())
        .mount("/", routes![signup, signin, get_user, sign_out])
}

#[post("/signup", format = "json", data = "<body>")]
async fn signup(db: LoyaltyDbConn, body: Json<UserSignup>) -> Result<(), APIError> {
    body.0.validate()?;

    db.run(move |c| {
        let new_value = NewUser {
            email: &body.0.email,
            name: &body.0.name,
            pass: &body.0.pass,
        };

        diesel::insert_into(db::schema::users::table)
            .values(&new_value)
            .execute(c)?;

        Ok(())
    })
    .await
}

#[post("/signin", format = "json", data = "<body>")]
async fn signin(
    cookies: &CookieJar<'_>,
    db: LoyaltyDbConn,
    body: Json<UserSignIn>,
) -> Result<status::Custom<&'static str>, APIError> {
    use db::schema::users::dsl::*;

    let fetched = db
        .run(move |c| {
            let req = body.0;

            users
                .filter(email.eq(req.email).and(pass.eq(req.pass)))
                .limit(1)
                .load::<db::models::UserFetch>(c)
        })
        .await?;

    if fetched.is_empty() {
        Ok(status::Custom(Status::Forbidden, "invalid credentials"))
    } else {
        let user = &fetched[0];
        cookies.add_private(Cookie::new("user_id", user.id.unwrap().to_string()));
        Ok(status::Custom(Status::Ok, "connected"))
    }
}

#[post("/signout")]
async fn sign_out(cookies: &CookieJar<'_>) -> status::Custom<&'static str> {
    cookies.remove_private(Cookie::named("user_id"));
    status::Custom(Status::Ok, "logged out")
}

#[derive(Debug)]
struct User(i32);

use rocket::async_trait;

#[crate::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = APIError;

    async fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|c| c.value().parse().ok())
            .map(|id| User(id))
            .or_forward(())
    }
}

#[get("/userinfo")]
async fn get_user(db: LoyaltyDbConn, user: User) -> Option<Json<db::models::UserFetch>> {
    use db::schema::users::dsl::*;
    let fetched = db
        .run(move |c| {
            users
                .filter(id.eq(user.0))
                .limit(1)
                .load::<db::models::UserFetch>(c)
        })
        .await;

    if fetched.is_err() {
        return None;
    }

    let mut elements: Vec<db::models::UserFetch> = fetched.unwrap();

    if elements.is_empty() {
        None
    } else {
        let found = elements.remove(0);
        Some(Json(found))
    }
}
