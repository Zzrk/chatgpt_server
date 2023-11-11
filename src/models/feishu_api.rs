use reqwest::Error;
use serde::{Deserialize, Serialize};

/// The body of get_tenant_access_token
#[derive(Serialize, Deserialize, Debug)]
struct GetTenantAccessTokenBody {
    app_id: String,
    app_secret: String,
}

/// The response of get_tenant_access_token
#[derive(Serialize, Deserialize, Debug)]
pub struct GetTenantAccessTokenResponse {
    pub code: i32,
    pub msg: String,
    pub tenant_access_token: String,
    pub expire: i32,
}

/// The type of the receive_id.
pub enum ReceiveIdType {
    OpenId,
    UserId,
    UnionId,
    Email,
    ChatId,
}

/// The body of send_message
#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageBody {
    pub receive_id: String,
    pub msg_type: String,
    pub content: String,
    pub uuid: Option<String>,
}

pub struct FeishuApi {
    pub base_url: String,
    pub app_id: String,
    pub app_secret: String,
}

impl FeishuApi {
    async fn get_tenant_access_token(
        &self,
        body: GetTenantAccessTokenBody,
    ) -> Result<String, Error> {
        let url = format!("{}/auth/v3/tenant_access_token/internal", self.base_url);

        let client = reqwest::Client::new();
        let body: GetTenantAccessTokenResponse =
            client.post(url).json(&body).send().await?.json().await?;
        let token = body.tenant_access_token;

        Ok(token)
    }

    pub async fn send_message(
        &self,
        receive_id_type: ReceiveIdType,
        body: SendMessageBody,
    ) -> Result<(), Error> {
        let token = self
            .get_tenant_access_token(GetTenantAccessTokenBody {
                app_id: self.app_id.clone(),
                app_secret: self.app_secret.clone(),
            })
            .await?;

        let receive_id_type = match receive_id_type {
            ReceiveIdType::OpenId => "open_id",
            ReceiveIdType::ChatId => "chat_id",
            ReceiveIdType::Email => "email",
            ReceiveIdType::UnionId => "union_id",
            ReceiveIdType::UserId => "user_id",
        };

        let url = format!(
            "{}/im/v1/messages?receive_id_type={}",
            self.base_url, receive_id_type
        );

        let client = reqwest::Client::new();
        client
            .post(url)
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await?;

        Ok(())
    }
}
