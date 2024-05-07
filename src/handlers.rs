use std::arch::x86_64::_rdrand64_step;
use redis::AsyncCommands;
use warp::{Rejection, Reply};
use warp::reply::json;
use enso_darknet::generate_uuid_v4;
use crate::{HealthcheckResponse, SDRequest};

type WebResult<T> = Result<T, Rejection>;

pub async fn health_checker_handler() -> WebResult<impl Reply>
{

    let uuid = generate_uuid_v4();

    let response = &HealthcheckResponse {
        status: false,
        uuid: uuid.clone(),
    };

    let client = redis::Client::open("redis://redis:6379/").unwrap();
    let mut publish_conn = client.get_tokio_connection().await.unwrap();

    publish_conn.publish::<&str, &str, i8>("render", uuid.as_str()).await.unwrap();

    Ok(json(response))
}

// A function to handle GET requests at /render
pub async fn render_handler() -> WebResult<impl Reply>
{
    let mut seed: u64 = 0;

    unsafe {
        _rdrand64_step(&mut seed);
    }
    // For simplicity, let's say we are returning a static post
    let response = &SDRequest {
        uuid: generate_uuid_v4(),
        prompt: String::from("Some static prompt!"),
        seed: seed as i64,
    };

    Ok(json(response))
}
