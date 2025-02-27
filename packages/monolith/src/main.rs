mod config;
mod galleries;
mod scraping_pipeline;
mod messages;
mod routes;
mod utils;

use axum::Router;
use config::{AppConfig, AxumConfig};
use scraping_pipeline::{AppModuleConnections, AppModules};
use tokio::net::TcpListener;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app_config = AppConfig::load().unwrap();
    let axum_config = app_config.axum_config.clone();
    let module_connections = AppModuleConnections::new();
    let router = routes::build_router(&app_config.axum_config, &module_connections);
    let app_modules = AppModules::init(app_config, module_connections).await.run();

    tracing::info!("App started");

    start_app(router, &axum_config).await;
}

async fn start_app(router: Router, axum_config: &AxumConfig) {
    let listener = TcpListener::bind(axum_config.host_addr.clone()).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
