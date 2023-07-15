use askama_axum::IntoResponse;
use axum::{http::StatusCode, Extension, Json};
use sqlx::PgPool;

use crate::model;

// GET /theme
pub async fn get_all_themes(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from themes".to_string();

    let themes = sqlx::query_as::<_, model::Theme>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(themes))
}

// GET /objective
pub async fn get_all_objectives(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from objectives".to_string();

    let objectives = sqlx::query_as::<_, model::Objective>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(objectives))
}

// GET /keyresult
pub async fn get_all_keyresults(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from keyresults".to_string();

    let keyresults = sqlx::query_as::<_, model::KeyResult>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(keyresults))
}

// GET /initiative
pub async fn get_all_initiatives(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from initiatives".to_string();

    let initiatives = sqlx::query_as::<_, model::Initiative>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(initiatives))
}

// GET /project
pub async fn get_all_projects(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from projects".to_string();

    let projects = sqlx::query_as::<_, model::Project>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(projects))
}

// GET /task
pub async fn get_all_tasks(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from tasks".to_string();

    let tasks = sqlx::query_as::<_, model::Task>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(tasks))
}

// GET /measurement
pub async fn get_all_measures(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from measures".to_string();

    let measurements = sqlx::query_as::<_, model::Measurement>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(measurements))
}
