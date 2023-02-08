use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePayload {
    pub action: String,
    pub messages: Vec<Message>,
    #[serde(rename = "conversation_id")]
    pub conversation_id: String,
    #[serde(rename = "parent_message_id")]
    pub parent_message_id: String,
    pub model: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: String,
    pub role: String,
    pub content: Content,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(rename = "content_type")]
    pub content_type: String,
    pub parts: Vec<String>,
}
