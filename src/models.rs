use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SDRequest {
    pub uuid: String,
    pub prompt: String,
    pub seed: i64,
}
