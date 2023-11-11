#[macro_use]
extern crate rocket;
use dotenv::dotenv;

mod models;
mod routes;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![index])
        .attach(routes::chatgpt::stage())
}
