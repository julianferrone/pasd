use askama_axum::IntoResponse;
use axum::{http::StatusCode, Extension, Json};
use sqlx::PgPool;

use crate::model;

pub async fn get_all_measures(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from measures".to_string();

    let measurements = sqlx::query_as::<_, model::Measurement>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(measurements))
}
