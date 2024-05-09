use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SDRequest {
    pub uuid: String,
    pub prompt: String,
    pub seed: i64,
}

#[derive(Serialize)]
pub struct HealthcheckResponse {
    pub status: bool,
    pub uuid: String,
    pub has_cuda: bool,
}
