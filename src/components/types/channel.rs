use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    pub label: String,
    pub id: i64,
}
