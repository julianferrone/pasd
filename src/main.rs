use axum::extract::Extension;
use axum::{routing::get, Router};

use sqlx::postgres::PgPoolOptions;
use std::fs;
use std::net::SocketAddr;
// use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace;
use tracing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
pub mod errors;
pub mod hypermedia_handlers;
pub mod model;
pub mod templater;

fn get_hypermedia_routes() -> Router {
    let hypermedia_router = Router::new()
        .route("/", get(hypermedia_handlers::get_root))
        .route("/theme", get(hypermedia_handlers::get_all_themes))
        .route("/theme/:theme_id", get(hypermedia_handlers::get_theme))
        .route("/objective", get(hypermedia_handlers::get_all_objectives))
        .route(
            "/objective/:objective_id",
            get(hypermedia_handlers::get_objective),
        )
        .route("/keyresult", get(hypermedia_handlers::get_all_keyresults))
        .route(
            "/keyresult/:keyresult_id",
            get(hypermedia_handlers::get_keyresult),
        )
        .route("/initiative", get(hypermedia_handlers::get_all_initiatives))
        .route(
            "/initiative/:initiative_id",
            get(hypermedia_handlers::get_initiative),
        )
        .route("/project", get(hypermedia_handlers::get_all_projects))
        .route(
            "/project/:project_id",
            get(hypermedia_handlers::get_project),
        )
        .route("/task/", get(hypermedia_handlers::get_all_tasks))
        .route("/task/:task_id", get(hypermedia_handlers::get_task))
        .route(
            "/measure/:measure_id",
            get(hypermedia_handlers::get_measure),
        );
    hypermedia_router
}

#[tokio::main]
async fn main() -> Result<(), String> {
    // start logging
    let tracing_layer = tracing_subscriber::fmt::layer();

    let filter = tracing_subscriber::filter::Targets::new()
        .with_target("tower_http::trace::on_response", tracing::Level::TRACE)
        .with_target("tower_http::trace::on_request", tracing::Level::TRACE)
        .with_target("tower_http::trace::make_span", tracing::Level::DEBUG)
        .with_default(tracing::Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .init();

    // open appstate.json
    let env = fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();
    assert_eq!(key, "DATABASE_URL");

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .map_err(|err| format!("ERROR: Could not connect to Postgres database: {err}"))?;

    // Routes for HATEOS
    let hypermedia_routes = get_hypermedia_routes();

    // Routes for Data API
    let data_routes = Router::new();

    // Serve
    let app = Router::new()
        .nest("/", hypermedia_routes)
        .nest("/api", data_routes)
        .layer(
            ServiceBuilder::new()
                .layer(Extension(pool))
                .layer(trace::TraceLayer::new_for_http()),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);

    // run our app with hyper, listening globally on port 3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// TODO:
// 1) convert into Rust web-server
// - Front-end: HTMX
// - Templating: Askana
// - Styling: Tailwind CSS
// - Back-end: Axum
// 2) Add undo/redo functionality using Command pattern
