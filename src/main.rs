#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;
 #[macro_use]
 extern crate diesel_migrations;

 use diesel::{QueryDsl, RunQueryDsl};
use diesel_migrations::{embed_migrations};
use ptb_api::configuration::get_configuration;
use ptb_api::models::User;
use ptb_api::schema::users::dsl::users;

// creates module for referencing migrations so you can put them in your binary
embed_migrations!();

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
    let connection = &configuration.database.get_connection();
    // This will run the necessary migrations.
    let answer = embedded_migrations::run(connection);

    let results = users
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());

    rocket::build().mount("/", routes![index, ping])
}
