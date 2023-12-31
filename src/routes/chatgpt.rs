use crate::models::{
    chatgpt_api::ChatGPTApi,
    feishu_api::{FeishuApi, ReceiveIdType, SendMessageBody},
    feishu_message::MessageReceiveRequest,
};
use rocket::{
    serde::json::{json, serde_json, Json, Value},
    State,
};
use std::{env, time::Duration};
use tokio::sync::Mutex;
use ttl_cache::TtlCache;

/// response for message receive
#[post("/", data = "<request>")]
async fn response(
    state: &State<Mutex<TtlCache<String, ()>>>,
    request: Json<MessageReceiveRequest>,
) -> Result<(), String> {
    // prevent the same message from being processed repeatedly
    let event_id: String = request.header.event_id.clone();
    let mut event_ids = state.inner().lock().await;
    if event_ids.contains_key(&event_id) {
        return Err("Duplicate event_id".into());
    }
    event_ids.insert(event_id, (), Duration::from_secs(7 * 3600 + 6 * 60));

    // answer the question from chatgpt
    let content = request.event.message.content.clone();
    let json_content: Value = serde_json::from_str(&content).unwrap();
    let question = json_content["text"].as_str().unwrap();

    let chatgpt_api = ChatGPTApi {
        base_url: env::var("OPENAI_API_URL").expect("CHATGPT_BASE_URL must be set"),
        api_key: env::var("OPENAI_API_KEY").expect("CHATGPT_API_KEY must be set"),
    };

    print!("\nquestion: {}\n", question);

    let answer = chatgpt_api
        .chat(question.to_string())
        .await
        .expect("chat failed");

    print!("\nanswer: {}\n", answer);

    // send message to feishu
    let sender = request.event.sender.clone();

    let feishu_api = FeishuApi {
        base_url: env::var("BASE_URL").expect("BASE_URL must be set"),
        app_id: env::var("APP_ID").expect("APP_ID must be set"),
        app_secret: env::var("APP_SECRET").expect("APP_SECRET must be set"),
    };

    // let body = SendMessageBody {
    //     receive_id: sender.sender_id.open_id,
    //     msg_type: "text".to_string(),
    //     content: json!({ "text": answer }).to_string(),
    //     uuid: None,
    // };

    // TODO: 使用消息卡片任然不支持代码块
    let body = SendMessageBody {
        receive_id: sender.sender_id.open_id,
        msg_type: "interactive".to_string(),
        content: json!({
          "config": {
            "wide_screen_mode": true
          },
          "elements": [
            {
              "tag": "markdown",
              "content": answer
            }
          ]
        })
        .to_string(),
        uuid: None,
    };

    feishu_api
        .send_message(ReceiveIdType::OpenId, body)
        .await
        .expect("send_message failed");

    Ok(())
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
