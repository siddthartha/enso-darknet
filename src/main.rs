mod models;
mod handlers;

use std::collections::HashMap;
use warp::{Filter};

use crate::models::SDRequest;
use crate::models::HealthcheckResponse;
use crate::handlers::{health_checker_handler, render_handler};

#[tokio::main]
async fn main()
{
    if std::env::var_os("RUST_LOG").is_none()
    {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    let health_checker_route = warp::path!("api" / "health")
        .and(warp::get())
        .and_then(health_checker_handler);

    let render_route = warp::path!("api" / "render")
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::get())
        .and_then(render_handler);

    let routes = health_checker_route
        .or(render_route)
        .with(warp::log("api"));

    println!("🚀 Enso ML API server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}
