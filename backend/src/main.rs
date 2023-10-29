mod api_result;
mod auth;
mod config;
mod domain;
mod error;
mod extractors;
mod init_tracing;
mod memory_view_repository;
mod routes;
mod state;
mod view_repositry_ext;

use crate::state::new_application_state;
use axum::{routing::get, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use snafu::{ResultExt, Whatever};
use tower_http::catch_panic::CatchPanicLayer;
use tracing::info;

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Whatever> {
    // initialize tracing
    init_tracing::init_tracing().whatever_context("failed to initialize tracing")?;

    let environment = std::env::var("ENVIRONMENT").whatever_context(
        "Please set ENVIRONMENT env var (probably you want to use either 'prod' or 'dev')",
    )?;

    let config =
        config::Config::load(&environment).whatever_context("Loading config has failed")?;

    info!("Resolved config: {:#?}", config);

    // build our application with a route
    let app = app(&config).await;

    info!("listening on {}", config.server.endpoint);
    axum::Server::bind(&config.server.endpoint)
        .serve(app.into_make_service())
        .await
        .whatever_context("failed to run server")?;

    Ok(())
}

async fn app(config: &config::Config) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/api", routes::make_api_router(&config.routes))
        .layer(CatchPanicLayer::custom(api_result::PanicHandler))
        // include trace context as header into the response
        .layer(OtelInResponseLayer)
        // start OpenTelemetry trace on incoming request
        .layer(OtelAxumLayer::default())
        // .layer(axum::middleware::from_fn(envelope_middleware))
        .with_state(new_application_state(config).await)
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
