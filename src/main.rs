use std::arch::x86_64::_rdrand64_step;
use serde::Serialize;
use warp::{reply::json, Filter, Rejection, Reply};
use uuid::Uuid;

type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: bool,
    pub uuid: String,
}

pub async fn health_checker_handler() -> WebResult<impl Reply>
{
    let mut low64_seed: u64 = 0;
    let mut high64_seed: u64 = 0;

    unsafe {
        _rdrand64_step(&mut low64_seed);
        _rdrand64_step(&mut high64_seed);
    }

    let response_json = &GenericResponse {
        status: false,
        uuid: Uuid::from_u64_pair(low64_seed, high64_seed).to_string(),
    };


    Ok(json(response_json))
}

#[tokio::main]
async fn main() {
    // if std::env::var_os("RUST_LOG").is_none() {
    //     std::env::set_var("RUST_LOG", "api=info");
    // }
    // pretty_env_logger::init();

    let health_checker = warp::path!("api" / "healthchecker")
        .and(warp::get())
        .and_then(health_checker_handler);

    let routes = health_checker.with(warp::log("api"));

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}
