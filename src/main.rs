use axum::extract::Extension;
use axum::{routing::get, Router};

use sqlx::postgres::PgPoolOptions;
use std::fs;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace;
use tracing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod errors;
pub mod handlers;
pub mod model;
pub mod templater;

fn get_hypermedia_routes() -> Router {
    let hypermedia_router = Router::new()
        .route("/", get(handlers::hypermedia::get_root))
        .route(
            "/theme",
            get(handlers::hypermedia::get_all_themes).post(handlers::hypermedia::add_theme),
        )
        .route("/theme/:theme_id", get(handlers::hypermedia::get_theme))
        .route(
            "/theme/:theme_id/objectives",
            get(handlers::hypermedia::get_theme_objectives),
        )
        .route(
            "/objective",
            get(handlers::hypermedia::get_all_objectives).post(handlers::hypermedia::add_objective),
        )
        .route(
            "/objective/:objective_id",
            get(handlers::hypermedia::get_objective),
        )
        .route(
            "/objective/:objective_id/keyresults",
            get(handlers::hypermedia::get_objective_keyresults),
        )
        .route(
            "/objective/:objective_id/initiatives",
            get(handlers::hypermedia::get_objective_initiatives),
        )
        .route(
            "/objective/:objective_id/projects",
            get(handlers::hypermedia::get_objective_projects),
        )
        .route(
            "/keyresult",
            get(handlers::hypermedia::get_all_keyresults).post(handlers::hypermedia::add_keyresult),
        )
        .route(
            "/keyresult/:keyresult_id",
            get(handlers::hypermedia::get_keyresult),
        )
        .route(
            "/initiative",
            get(handlers::hypermedia::get_all_initiatives)
                .post(handlers::hypermedia::add_initiative),
        )
        .route(
            "/initiative/:initiative_id",
            get(handlers::hypermedia::get_initiative),
        )
        .route(
            "/project",
            get(handlers::hypermedia::get_all_projects).post(handlers::hypermedia::add_project),
        )
        .route(
            "/project/:project_id",
            get(handlers::hypermedia::get_project),
        )
        .route("/task/", get(handlers::hypermedia::get_all_tasks))
        .route("/task/:task_id", get(handlers::hypermedia::get_task))
        .route(
            "/measure/:measure_id",
            get(handlers::hypermedia::get_measure),
        );
    hypermedia_router
}

fn get_static_asset_routes() -> Router {
    let static_assets_router = Router::new()
        .route("/js/htmx.min.js", get(handlers::assets::htmx_js))
        .route("/js/json-enc.js", get(handlers::assets::htmx_ext_json_js));
    static_assets_router
}

fn get_data_routes() -> Router {
    let data_router = Router::new();
    data_router
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

    // Serve
    let app = Router::new()
        .nest("/", get_hypermedia_routes())
        .nest("/api", get_data_routes())
        .nest("/static", get_static_asset_routes())
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
// - Templating: Askama
// - Styling: Tailwind CSS
// - Back-end: Axum
// 2) Add undo/redo functionality using Command pattern
