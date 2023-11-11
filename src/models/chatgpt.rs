use serde::{Deserialize, Serialize};

/// request for connect
#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectRequest {
    pub challenge: String,
    token: String,
}

/// response for connect
#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectResponse {
    pub challenge: String,
}
