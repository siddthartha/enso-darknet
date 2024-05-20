use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SDRequest {
    pub uuid: String,
    pub prompt: String,
    pub seed: i64,
    pub width: u32,
    pub height: u32,
    pub steps: u8,
    pub intermediates: bool,
    pub version: u8,
}

#[derive(Serialize)]
pub struct HealthcheckResponse {
    pub status: bool,
    pub uuid: String,
    pub has_cuda: bool,
    pub cuda_devices_count: i32,
    pub has_cudann: bool,
    pub has_mps: bool,
    pub has_vulkan: bool,
    pub has_openmp: bool,

}
