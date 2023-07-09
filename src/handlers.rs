use crate::{model, templater};
use axum::{extract, http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::PgPool;

// GET /
pub async fn get_root() {}

// macro_rules! get_all_from_table{
//     ($table_name:expr, $struct:item)=>{
//         pub async fn get_all_$table_name(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
//             let sql = format!("SELECT * from {table_name}").to_string();
//             let $table_name = sql::query_as::<_,
//         }
//     }
// }
// get_all_from_table!("themes".to_string(), model::Theme);

// GET /theme/
pub async fn get_all_themes(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from themes".to_string();

    let themes = sqlx::query_as::<_, model::Theme>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(themes))
}

// GET /objective/
pub async fn get_all_objectives(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from objectives".to_string();

    let objectives = sqlx::query_as::<_, model::Objective>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(objectives))
}

// GET /keyresult/
pub async fn get_all_keyresults(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from keyresults".to_string();

    let keyresults = sqlx::query_as::<_, model::KeyResult>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(keyresults))
}

// GET /initiative/
pub async fn get_all_initiatives(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from initiatives".to_string();

    let initiatives = sqlx::query_as::<_, model::Initiative>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(initiatives))
}

// GET /project/
pub async fn get_all_projects(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from projects".to_string();

    let projects = sqlx::query_as::<_, model::Project>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(projects))
}

// GET /task/
pub async fn get_all_tasks(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from tasks".to_string();

    let tasks = sqlx::query_as::<_, model::Task>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(tasks))
}

// GET /measure/
pub async fn get_all_measures(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from measures".to_string();

    let measurements = sqlx::query_as::<_, model::Measurement>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(measurements))
}

// GET /theme/id
pub async fn get_theme(
    Extension(pool): Extension<PgPool>,
    extract::Path(theme_id): extract::Path<i32>,
) -> axum::response::Response {
    let theme_row = sqlx::query!(
        r#"SELECT title, theme_status as "theme_status: model::Status" from themes where theme_id = $1"#,
        &theme_id
    )
    .fetch_one(&pool)
    .await;

    let rows: Result<Vec<model::Objective>, _> = sqlx::query_as(
        r#"SELECT *
        FROM objectives
        where objectives.theme_id = $1
        ORDER BY objectives.objective_id;"#,
    )
    .bind(&theme_id)
    .fetch_all(&pool)
    .await;

    match theme_row {
        Ok(theme) => {
            let objs: Option<Vec<model::Objective>> = rows.ok();
            let template =
                templater::ThemeTemplate::new(theme.title, theme.theme_status, objs.clone());
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template =
                templater::ErrorTemplate::new(StatusCode::NOT_FOUND, "Theme Not Found".to_string());
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// GET /objective/id
pub async fn get_objective(
    Extension(pool): Extension<PgPool>,
    extract::Path(objective_id): extract::Path<i32>,
) -> axum::response::Response {
    let obj_row = sqlx::query!(
        r#"
        SELECT objectives.title, themes.theme_id, themes.title as theme_title 
        from objectives 
        LEFT JOIN themes 
        ON objectives.theme_id = themes.theme_id 
        where objectives.objective_id = $1;
        "#,
        objective_id
    )
    .fetch_one(&pool)
    .await;

    let kr_rows: Result<Vec<model::KeyResult>, _> = sqlx::query_as(
        r#"SELECT *
        FROM keyresults
        where keyresults.objective_id = $1
        ORDER BY keyresults.keyresult_id;"#,
    )
    .bind(&objective_id)
    .fetch_all(&pool)
    .await;

    let ini_rows: Result<Vec<model::Initiative>, _> = sqlx::query_as(
        r#"SELECT *
        FROM initiatives
        where initiatives.objective_id = $1
        ORDER BY initiatives.initiative_id;"#,
    )
    .bind(&objective_id)
    .fetch_all(&pool)
    .await;

    let proj_rows: Result<Vec<model::Project>, _> = sqlx::query_as(
        r#"SELECT *
        FROM projects
        where projects.objective_id = $1
        ORDER BY projects.project_id;"#,
    )
    .bind(&objective_id)
    .fetch_all(&pool)
    .await;

    match obj_row {
        Ok(row) => {
            let template = templater::ObjectiveTemplate::new(
                row.title,
                row.theme_id,
                row.theme_title,
                kr_rows.ok(),
                ini_rows.ok(),
                proj_rows.ok(),
            );
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template = templater::ErrorTemplate::new(
                StatusCode::NOT_FOUND,
                "Objective Not Found".to_string(),
            );
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// GET /project/id
pub async fn get_project() {}

// GET /task/id
pub async fn get_task() {}

// GET /measure/id
pub async fn get_measure() {}

// POST /theme
pub async fn add_theme() {}

// POST /objective
pub async fn add_objective() {}

// POST /project
pub async fn add_project() {}

// POST /task
pub async fn add_task() {}

// POST /measure
pub async fn add_measure() {}

// DELETE /theme/id
pub async fn remove_theme() {}

// DELETE /objective/id
pub async fn remove_objective() {}

// DELETE /project/id
pub async fn remove_project() {}

// DELETE /task/id
pub async fn remove_task() {}

// DELETE /measure/id
pub async fn remove_measure() {}

// PUT /theme/id
pub async fn update_theme() {}

// PUT /objective/id
pub async fn update_objective() {}

// PUT /project/id
pub async fn update_project() {}

// PUT /task/id
pub async fn update_task() {}

// PUT /measure/id
pub async fn update_measure() {}
