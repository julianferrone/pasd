use axum::extract::Extension;
// use axum::http::{StatusCode, Uri};
use axum::{
    routing::{delete, get, post},
    Router,
};

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
            get(handlers::hypermedia::get_root_themes).post(handlers::hypermedia::add_theme),
        )
        .route("/theme/:theme_id", get(handlers::hypermedia::get_theme))
        .route(
            "/theme/:theme_id/objectives",
            get(handlers::hypermedia::get_theme_objectives),
        )
        .route(
            "/theme/:theme_id/row",
            get(handlers::hypermedia::get_theme_row),
        )
        .route(
            "/theme/:theme_id/form",
            get(handlers::hypermedia::get_theme_form),
        )
        .route("/objective", post(handlers::hypermedia::add_objective))
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
            "/objective/:objective_id/row",
            get(handlers::hypermedia::get_objective_row),
        )
        .route(
            "/objective/:objective_id/form",
            get(handlers::hypermedia::get_objective_form),
        )
        .route("/keyresult", post(handlers::hypermedia::add_keyresult))
        .route(
            "/keyresult/:keyresult_id",
            get(handlers::hypermedia::get_keyresult),
        )
        .route(
            "/keyresult/:keyresult_id/row",
            get(handlers::hypermedia::get_keyresult_row),
        )
        .route(
            "/keyresult/:keyresult_id/form",
            get(handlers::hypermedia::get_keyresult_form),
        )
        .route("/initiative", post(handlers::hypermedia::add_initiative))
        .route(
            "/initiative/:initiative_id",
            get(handlers::hypermedia::get_initiative),
        )
        .route(
            "/initiative/:initiative_id/row",
            get(handlers::hypermedia::get_initiative_row),
        )
        .route(
            "/initiative/:initiative_id/form",
            get(handlers::hypermedia::get_initiative_form),
        )
        .route("/project", post(handlers::hypermedia::add_project))
        .route(
            "/project/:project_id",
            get(handlers::hypermedia::get_project),
        )
        .route(
            "/project/:project_id/row",
            get(handlers::hypermedia::get_project_row),
        )
        .route(
            "/project/:project_id/form",
            get(handlers::hypermedia::get_project_form),
        )
        .route("/task", post(handlers::hypermedia::add_task))
        .route("/task/:task_id", get(handlers::hypermedia::get_task))
        .route(
            "/task/:task_id/row",
            get(handlers::hypermedia::get_task_row),
        )
        .route(
            "/task/:task_id/form",
            get(handlers::hypermedia::get_task_form),
        )
        .route("/measure", post(handlers::hypermedia::add_measure))
        .route(
            "/measure/:measure_id",
            get(handlers::hypermedia::get_measure),
        )
        .route(
            "/measure/:measure_id/row",
            get(handlers::hypermedia::get_measure_row),
        )
        .route(
            "/measure/:measure_id/form",
            get(handlers::hypermedia::get_measure_form),
        )
        .route(
            "/:resource/:resource_id",
            delete(handlers::hypermedia::remove_resource),
        );
    hypermedia_router
}

fn get_static_asset_routes() -> Router {
    let static_assets_router = Router::new()
        .route("/js/htmx.min.js", get(handlers::assets::htmx_js))
        .route("/js/json-enc.js", get(handlers::assets::htmx_ext_json_js))
        .route(
            "/js/hyperscript.min.js",
            get(handlers::assets::hyperscript_js),
        );
    static_assets_router
}

fn get_data_routes() -> Router {
    let data_router = Router::new()
        .route("/theme", get(handlers::data::get_all_themes))
        .route("/objective", get(handlers::data::get_all_objectives))
        .route("/keyresult", get(handlers::data::get_all_keyresults))
        .route("/initiative", get(handlers::data::get_all_initiatives))
        .route("/project", get(handlers::data::get_all_projects))
        .route("/task", get(handlers::data::get_all_tasks))
        .route("/measurement", get(handlers::data::get_all_measures));
    data_router
}

// async fn fallback(uri: Uri) -> (StatusCode, String) {
//     (StatusCode::NOT_FOUND, format!("No route for {}", uri))
// }

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
        // .fallback(handlers::hypermedia::get_error_404_page)
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
