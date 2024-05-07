mod models;

use std::arch::x86_64::_rdrand64_step;
use serde::Serialize;
use warp::{reply::json, Filter, Rejection, Reply};
use uuid::Uuid;
use redis;
use redis::{AsyncCommands};

use models::SDRequest;

type WebResult<T> = Result<T, Rejection>;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: bool,
    pub uuid: String,
}

pub fn generate_uuid_v4() -> String
{
    let mut low64_seed: u64 = 0;
    let mut high64_seed: u64 = 0;

    unsafe {
        _rdrand64_step(&mut low64_seed);
        _rdrand64_step(&mut high64_seed);
    }

    let uuid = Uuid::from_u64_pair(low64_seed, high64_seed).to_string().clone();
    uuid
}

pub async fn health_checker_handler() -> WebResult<impl Reply>
{
    let mut low64_seed: u64 = 0;
    let mut high64_seed: u64 = 0;

    unsafe {
        _rdrand64_step(&mut low64_seed);
        _rdrand64_step(&mut high64_seed);
    }

    let uuid = Uuid::from_u64_pair(low64_seed, high64_seed).to_string().clone();

    let response_json = &GenericResponse {
        status: false,
        uuid: uuid.clone(),
    };

    let client = redis::Client::open("redis://redis:6379/").unwrap();
    let mut publish_conn = client.get_tokio_connection().await.unwrap();

    publish_conn.publish::<&str, &str, i8>("render", uuid.as_str()).await.unwrap();

    Ok(json(response_json))
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

#[tokio::main]
async fn main()
{
    if std::env::var_os("RUST_LOG").is_none()
    {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    let health_checker_route = warp::path!("api" / "healthchecker")
        .and(warp::get())
        .and_then(health_checker_handler);

    let render_route = warp::path!("api" / "sd" / "render")
        .and(warp::get())
        .and_then(render_handler);

    let routes = health_checker_route
        .with(warp::log("api"))
        .or(render_route);

    println!("🚀 Enso ML API server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}
