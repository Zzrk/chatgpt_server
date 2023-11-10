use rocket::serde::json::{json, Json, Value};

#[get("/")]
async fn get_data() -> &'static str {
    "Hello!"
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("ChatGPT", |rocket| async {
        rocket
            .mount("/chatgpt", routes![get_data])
            .register("/chatgpt", catchers![not_found])
    })
}
