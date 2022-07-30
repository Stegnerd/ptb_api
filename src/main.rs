#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;

use diesel::{EqAll, QueryDsl, RunQueryDsl};
use ptb_api::establish_connection;
use ptb_api::models::User;
use ptb_api::schema::users::dsl::users;
use ptb_api::schema::users::email;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/ping")]
fn ping() -> &'static str {
    "Pong"
}

#[launch]
fn rocket() -> _ {
    println!("starting up the api");

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());


    rocket::build()
        .mount("/", routes![index, ping])
}
