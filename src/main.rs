use axum::extract::Extension;
use axum::{routing::get, Router};

use sqlx::postgres::PgPoolOptions;
use std::fs;
// use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace;
pub mod errors;
pub mod handlers;
pub mod model;
pub mod templater;

#[tokio::main]
async fn main() -> Result<(), String> {
    // open appstate.json
    let env = fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();
    assert_eq!(key, "DATABASE_URL");

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .map_err(|err| format!("ERROR: Could not connect to Postgres database: {err}"))?;

    // build our application with a route
    let app = Router::new()
        .route("/", get(handlers::get_root))
        .route("/theme", get(handlers::get_all_themes))
        .route("/theme/:theme_id", get(handlers::get_theme))
        .route("/objective", get(handlers::get_all_objectives))
        .route("/objective/:objective_id", get(handlers::get_objective))
        .route("/projects", get(handlers::get_all_projects))
        .route("/project/:project_id", get(handlers::get_project))
        .route("/task/", get(handlers::get_all_tasks))
        .route("/task/:task_id", get(handlers::get_task))
        .route("/measure/:measure_id", get(handlers::get_measure))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(pool))
                .layer(trace::TraceLayer::new_for_http()),
        );

    // .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
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
