use std::arch::x86_64::_rdrand64_step;
use std::collections::HashMap;
use redis::AsyncCommands;
use warp::{reject, Rejection, Reply};
use warp::reply::{json};
use enso_darknet::{DEFAULT_STEPS, generate_uuid_v4, RENDER_QUEUE, STEPS_LIMIT};
use serde_json;
use crate::{HealthcheckResponse, SDRequest};

type WebResult<T> = Result<T, Rejection>;

pub async fn health_checker_handler() -> WebResult<impl Reply>
{
    let uuid = generate_uuid_v4();

    let response = &HealthcheckResponse {
        status: false,
        uuid: uuid.clone(),
        has_cuda: tch::Cuda::is_available(),
        cuda_devices_count: tch::Cuda::device_count() as i32,
        has_cudann: tch::Cuda::cudnn_is_available(),
        has_mps: tch::utils::has_mps(),
        has_vulkan: tch::utils::has_vulkan(),
        has_openmp: tch::utils::has_openmp(),
    };

    Ok(json(response))
}

// A function to handle GET requests at /render
pub async fn render_handler(q: HashMap<String, String>) -> WebResult<impl Reply>
{
    let mut random_seed: u64 = 0;

    unsafe {
        _rdrand64_step(&mut random_seed);
    }

    return match q.get("prompt") {
        None => {
            Err(reject::reject())
        }
        Some(prompt) => {
            let request = &SDRequest {
                uuid: generate_uuid_v4(),
                prompt: prompt.clone(),
                seed: match q.get("seed") {
                    None => { random_seed as i64 }
                    Some(seed) => { seed.parse::<i64>().unwrap() as i64 }
                },
                width: match q.get("width") {
                    None => 768,
                    Some(width) => { width.parse::<u32>().unwrap() as u32 }
                },
                height: match q.get("height") {
                    None => 768,
                    Some(height) => { height.parse::<u32>().unwrap() as u32 }
                },
                steps: match q.get("steps") {
                    None => DEFAULT_STEPS,
                    Some(steps) => {
                        (steps.parse::<u8>().unwrap() as u8)
                            .min(STEPS_LIMIT)
                    }
                }
            };

            let client = redis::Client::open(enso_darknet::redis_host()).unwrap();
            let mut publish_conn = client.get_tokio_connection().await.unwrap();

            publish_conn.publish::<&str, &str, i8>(
                RENDER_QUEUE,
                serde_json::to_string(request).unwrap().as_str()
            ).await.unwrap();

            Ok(json(request))
        }
    }
}
