#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;

use diesel::{ QueryDsl, RunQueryDsl};
use ptb_api::configuration::get_configuration;
use ptb_api::models::User;
use ptb_api::schema::users::dsl::users;

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

    // let connection = establish_connection();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let results = users
        .limit(5)
        .load::<User>(&configuration.database.get_connection())
        .expect("Error loading users");

    println!("Displaying {} users", results.len());


    rocket::build()
        .mount("/", routes![index, ping])
}
