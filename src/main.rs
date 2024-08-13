use std::env;

use dotenvy::dotenv;
use tracing::warn;
use tracing_subscriber::EnvFilter;

mod db;
mod routing;
fn init_logging() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_thread_names(true)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(true)
        .init();
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    init_logging();
    let port: String = env::var("PORT").unwrap_or_else(|_| {
        warn!("PORT not set, using default: 3000");
        String::from("3000")
    });
    routing::run_server(port).await;
}
