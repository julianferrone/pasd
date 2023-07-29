use axum::extract::Extension;
// use axum::http::{StatusCode, Uri};
use axum::{
    routing::{delete, get, post, put},
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
use handlers::{assets, data, hypermedia};
pub mod model;
pub mod templater;

fn get_hypermedia_routes() -> Router {
    let hypermedia_router = Router::new()
        .route("/", get(hypermedia::get_root))
        .route(
            "/theme",
            get(hypermedia::get_root_themes).post(hypermedia::add_theme),
        )
        .route(
            "/theme/:theme_id",
            get(hypermedia::get_theme).put(hypermedia::update_theme),
        )
        .route(
            "/theme/:theme_id/objectives",
            get(hypermedia::get_theme_objectives),
        )
        .route("/theme/:theme_id/row", get(hypermedia::get_theme_row))
        .route("/theme/:theme_id/form", get(hypermedia::get_theme_form))
        .route("/objective", post(hypermedia::add_objective))
        .route(
            "/objective/:objective_id",
            get(hypermedia::get_objective).put(hypermedia::update_objective),
        )
        .route(
            "/objective/:objective_id/keyresults",
            get(hypermedia::get_objective_keyresults),
        )
        .route(
            "/objective/:objective_id/initiatives",
            get(hypermedia::get_objective_initiatives),
        )
        .route(
            "/objective/:objective_id/projects",
            get(hypermedia::get_objective_projects),
        )
        .route(
            "/objective/:objective_id/row",
            get(hypermedia::get_objective_row),
        )
        .route(
            "/objective/:objective_id/form",
            get(hypermedia::get_objective_form),
        )
        .route("/keyresult", post(hypermedia::add_keyresult))
        .route(
            "/keyresult/:keyresult_id",
            get(hypermedia::get_keyresult).put(hypermedia::update_keyresult),
        )
        .route(
            "/keyresult/:keyresult_id/row",
            get(hypermedia::get_keyresult_row),
        )
        .route(
            "/keyresult/:keyresult_id/form",
            get(hypermedia::get_keyresult_form),
        )
        .route(
            "/keyresult/:keyresult_id/measures",
            get(hypermedia::get_keyresult_measurements)
        )
        .route("/initiative", post(hypermedia::add_initiative))
        .route(
            "/initiative/:initiative_id",
            get(hypermedia::get_initiative).put(hypermedia::update_initiative),
        )
        .route(
            "/initiative/:initiative_id/row",
            get(hypermedia::get_initiative_row),
        )
        .route(
            "/initiative/:initiative_id/form",
            get(hypermedia::get_initiative_form),
        )
        .route("/project", post(hypermedia::add_project))
        .route(
            "/project/:project_id",
            get(hypermedia::get_project).put(hypermedia::update_project),
        )
        .route("/project/:project_id/row", get(hypermedia::get_project_row))
        .route(
            "/project/:project_id/form",
            get(hypermedia::get_project_form),
        )
        .route(
            "/project/:project_id/tasks",
            get(hypermedia::get_project_tasks)
        )
        .route("/task", post(hypermedia::add_task))
        .route(
            "/task/:task_id",
            get(hypermedia::get_task).put(hypermedia::update_task),
        )
        .route("/task/:task_id/row", get(hypermedia::get_task_row))
        .route("/task/:task_id/form", get(hypermedia::get_task_form))
        .route("/measure", post(hypermedia::add_measure))
        .route(
            "/measure/:measure_id",
            put(hypermedia::update_measure),
        )
        .route("/measure/:measure_id/row", get(hypermedia::get_measure_row))
        .route(
            "/measure/:measure_id/form",
            get(hypermedia::get_measure_form),
        )
        .route(
            "/:resource/:resource_id",
            delete(hypermedia::remove_resource),
        );
    hypermedia_router
}

fn get_static_asset_routes() -> Router {
    let static_assets_router = Router::new()
        .route("/img/favicon.ico", get(assets::favicon))
        .route("/js/json-enc.js", get(assets::htmx_ext_json_js))
        .route("/js/htmx.min.js", get(assets::htmx_js))
        .route("/js/hyperscript.min.js", get(assets::hyperscript_js))
        .route("/js/sweetalert2.min.js", get(assets::sweetalert_2_js));
    static_assets_router
}

fn get_data_routes() -> Router {
    let data_router = Router::new()
        .route("/theme", get(data::get_all_themes))
        .route("/objective", get(data::get_all_objectives))
        .route("/keyresult", get(data::get_all_keyresults))
        .route("/initiative", get(data::get_all_initiatives))
        .route("/project", get(data::get_all_projects))
        .route("/task", get(data::get_all_tasks))
        .route("/measurement", get(data::get_all_measures));
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
        // .fallback(hypermedia::get_error_404_page)
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
