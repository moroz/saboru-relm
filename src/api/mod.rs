use crate::components::{sidebar_item::Channel, types::ChatMessage};

pub struct API;

const BASE_URL: &str = "http://localhost:3000/api";

impl API {
    pub async fn fetch_channels() -> Result<Vec<Channel>, serde_json::Error> {
        let url = format!("{BASE_URL}/channels");
        let res = reqwest::get(url).await.unwrap().text().await.unwrap();
        serde_json::from_str(&res)
    }

    pub async fn fetch_messages(channel_id: i64) -> Result<Vec<ChatMessage>, serde_json::Error> {
        let url = format!("{BASE_URL}/messages/{channel_id}");
        let res = reqwest::get(url).await.unwrap().text().await.unwrap();
        serde_json::from_str(&res)
    }
}
