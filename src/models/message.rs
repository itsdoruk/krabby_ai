#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ApiResponse {
    pub choices: Vec<Choice>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Choice {
    pub message: ResponseMessage,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ResponseMessage {
    pub content: String,
    pub role: String,
}