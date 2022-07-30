#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;

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

    rocket::build()
        .mount("/", routes![index, ping])
}
