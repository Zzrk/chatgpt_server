use rocket::serde::json::{json, Json, Value};

use crate::models::chatgpt::{ConnectRequest, ConnectResponse};

/// response for connect
#[post("/", data = "<request>")]
async fn response(request: Json<ConnectRequest>) -> Result<Json<ConnectResponse>, String> {
    let challenge = request.challenge.clone();
    let response = ConnectResponse { challenge };
    Ok(Json(response))
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
            .mount("/chatgpt", routes![response])
            .register("/chatgpt", catchers![not_found])
    })
}
