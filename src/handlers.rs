use std::arch::x86_64::_rdrand64_step;
use std::collections::HashMap;
use redis::AsyncCommands;
use warp::{reject, Rejection, Reply};
use warp::reply::{json};
use enso_darknet::{generate_uuid_v4, RENDER_QUEUE};
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
    };

    Ok(json(response))
}

// A function to handle GET requests at /render
pub async fn render_handler(q: HashMap<String, String>) -> WebResult<impl Reply>
{
    let mut seed: u64 = 0;

    unsafe {
        _rdrand64_step(&mut seed);
    }

    return match q.get("prompt") {
        None => {
            Err(reject::reject())
        }
        Some(prompt) => {
            let request = &SDRequest {
                uuid: generate_uuid_v4(),
                prompt: prompt.clone(),
                seed: seed as i64,
            };

            let client = redis::Client::open("redis://redis:6379/").unwrap();
            let mut publish_conn = client.get_tokio_connection().await.unwrap();

            publish_conn.publish::<&str, &str, i8>(
                RENDER_QUEUE,
                serde_json::to_string(request).unwrap().as_str()
            ).await.unwrap();

            Ok(json(request))
        }
    }
}
