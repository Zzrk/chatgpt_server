use serde::{Deserialize, Serialize};

/// request for message receive
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReceiveRequest {
    /// 事件模式
    pub schema: String,
    /// 事件头
    pub header: MessageReceiveRequestHeader,
    /// 事件体
    pub event: MessageReceiveRequestEvent,
}

/// header of message receive
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReceiveRequestHeader {
    /// 事件 ID
    pub event_id: String,
    /// 事件类型：im.message.receive_v1
    pub event_type: String,
    /// 事件创建时间戳（单位：毫秒）
    pub create_time: String,
    /// 事件 Token
    pub token: String,
    /// 应用 ID
    pub app_id: String,
    /// 租户 Key
    pub tenant_key: String,
}

/// event of message receive
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReceiveRequestEvent {
    /// 事件的发送者
    sender: MessageReceiveRequestSender,
    /// 事件中包含的消息内容
    message: MessageReceiveRequestMessage,
}

/// sender of message receive event
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReceiveRequestSender {
    /// 用户 ID
    pub sender_id: UserId,
    /// 消息发送者类型。目前只支持用户(user)发送的消息。
    pub sender_type: String,
    /// tenant key，为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用里面的唯一标识
    pub tenant_key: String,
}

/// user_ids
#[derive(Serialize, Deserialize, Debug)]
pub struct UserId {
    pub union_id: String,
    pub user_id: String,
    pub open_id: String,
}

/// message of message receive event
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReceiveRequestMessage {
    /// 消息的open_message_id
    message_id: String,
    /// 根消息id，用于回复消息场景
    root_id: Option<String>,
    /// 父消息的id，用于回复消息场景
    parent_id: Option<String>,
    /// 消息发送时间（毫秒）
    create_time: String,
    /// 消息更新时间（毫秒）
    update_time: String,
    /// 消息所在的群组 ID
    chat_id: String,
    /// 消息所在的群组类型，p2p 为单聊，group 为群聊
    chat_type: String,
    /// 消息类型
    message_type: String,
    /// 消息内容, JSON 格式
    content: String,
    /// 被提及用户的信息
    mentions: Option<Vec<MessageReceiveRequestMessageMention>>,
    /// 用户代理数据
    user_agent: Option<String>,
}

/// mention of message receive event
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReceiveRequestMessageMention {
    /// mention key
    key: String,
    /// 用户 ID
    id: UserId,
    /// 用户姓名
    name: String,
    /// tenant key，为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用里面的唯一标识
    tenant_key: String,
}
