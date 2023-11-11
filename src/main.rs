#[macro_use]
extern crate rocket;

mod models;
mod routes;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(routes::chatgpt::stage())
}
