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

    let root_route = warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("./gui/index.html"));

    let download_route = warp::path("result")
        .and(warp::fs::dir("./media/"));

    let gui_route = warp::path("gui")
        .and(warp::fs::dir("./gui/"));

    let routes = health_checker_route
        .or(render_route)
        .or(download_route)
        .or(gui_route)
        .or(root_route)
        .with(warp::log("api"));

    println!("🚀 Enso ML API server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}
