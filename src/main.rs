#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use tokio::sync::Mutex;
use ttl_cache::TtlCache;

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
        .manage(Mutex::new(TtlCache::<String, ()>::new(usize::MAX)))
        .mount("/", routes![index])
        // route for first connect test
        // .attach(routes::connect::stage())
        // route for chatgpt
        .attach(routes::chatgpt::stage())
}
