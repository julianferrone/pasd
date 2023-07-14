use crate::{model, templater};
use axum::{
    extract,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension, Json,
};
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
        .await;

    let template = templater::AllThemesTemplate::new(themes.ok().clone());
    templater::HtmlTemplate(template).into_response()
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
pub async fn get_all_measures() {}

// GET /theme/:theme_id
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

    match theme_row {
        Ok(theme) => {
            let objectives: Option<Vec<model::Objective>> = sqlx::query_as(
                r#"SELECT *
                FROM objectives
                where objectives.theme_id = $1
                ORDER BY objectives.objective_id;"#,
            )
            .bind(&theme_id)
            .fetch_all(&pool)
            .await
            .ok();
            let template = templater::ThemeTemplate::new(
                theme.title,
                theme_id,
                theme.theme_status,
                objectives.clone(),
            );
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template =
                templater::ErrorTemplate::new(StatusCode::NOT_FOUND, "Theme Not Found".to_string());
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// GET /theme/:theme_id/objectives
pub async fn get_theme_objectives(
    Extension(pool): Extension<PgPool>,
    extract::Path(theme_id): extract::Path<i32>,
) -> axum::response::Response {
    let objectives: Option<Vec<model::Objective>> = sqlx::query_as(
        r#"SELECT *
        FROM objectives
        where objectives.theme_id = $1
        ORDER BY objectives.objective_id;"#,
    )
    .bind(&theme_id)
    .fetch_all(&pool)
    .await
    .ok();
    let template = templater::ListObjectivesTemplate::new(
        objectives.clone(),
    );
    templater::HtmlTemplate(template).into_response()
}

// GET /objective/:objective_id
pub async fn get_objective(
    Extension(pool): Extension<PgPool>,
    extract::Path(objective_id): extract::Path<i32>,
) -> axum::response::Response {
    let objective_row = sqlx::query!(
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

    match objective_row {
        Ok(obj) => {
            let kr_rows: Option<Vec<model::KeyResult>> = sqlx::query_as(
                r#"SELECT *
                FROM keyresults
                where keyresults.objective_id = $1
                ORDER BY keyresults.keyresult_id;"#,
            )
            .bind(&objective_id)
            .fetch_all(&pool)
            .await
            .ok();

            let ini_rows: Option<Vec<model::Initiative>> = sqlx::query_as(
                r#"SELECT *
                FROM initiatives
                where initiatives.objective_id = $1
                ORDER BY initiatives.initiative_id;"#,
            )
            .bind(&objective_id)
            .fetch_all(&pool)
            .await
            .ok();

            let proj_rows: Option<Vec<model::Project>> = sqlx::query_as(
                r#"SELECT *
                FROM projects
                where projects.objective_id = $1
                ORDER BY projects.project_id;"#,
            )
            .bind(&objective_id)
            .fetch_all(&pool)
            .await
            .ok();

            let template = templater::ObjectiveTemplate::new(
                obj.title,
                obj.theme_id,
                obj.theme_title,
                kr_rows,
                ini_rows,
                proj_rows,
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

// GET /keyresult/:keyresult_id
pub async fn get_keyresult(
    Extension(pool): Extension<PgPool>,
    extract::Path(keyresult_id): extract::Path<i32>,
) -> axum::response::Response {
    let keyresult_row = sqlx::query!(
        r#"
        SELECT keyresults.title, objectives.objective_id, objectives.title as objective_title 
        FROM keyresults 
        LEFT JOIN objectives 
        ON keyresults.objective_id = objectives.objective_id 
        WHERE keyresults.keyresult_id = $1;
        "#,
        keyresult_id
    )
    .fetch_one(&pool)
    .await;

    match keyresult_row {
        Ok(keyresult) => {
            let measurements: Option<Vec<model::Measurement>> = sqlx::query_as(
                r#"SELECT *
                FROM measurements
                WHERE measurements.keyresult_id = $1
                ORDER BY measurements.measurement_id;"#,
            )
            .bind(&keyresult_id)
            .fetch_all(&pool)
            .await
            .ok();

            let template = templater::KeyResultTemplate::new(
                keyresult.title,
                keyresult.objective_id,
                keyresult.objective_title,
                measurements,
            );
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template = templater::ErrorTemplate::new(
                StatusCode::NOT_FOUND,
                "Key Result Not Found".to_string(),
            );
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// GET /initiative/:initiative_id
pub async fn get_initiative(
    Extension(pool): Extension<PgPool>,
    extract::Path(initiative_id): extract::Path<i32>,
) -> axum::response::Response {
    let initiative_row = sqlx::query!(
        r#"
        SELECT initiatives.title, objectives.objective_id, objectives.title as objective_title 
        FROM initiatives 
        LEFT JOIN objectives 
        ON initiatives.objective_id = objectives.objective_id 
        WHERE initiatives.initiative_id = $1;
        "#,
        initiative_id
    )
    .fetch_one(&pool)
    .await;

    match initiative_row {
        Ok(initiative) => {
            let template = templater::InitiativeTemplate::new(
                initiative.title,
                initiative.objective_id,
                initiative.objective_title,
            );
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template = templater::ErrorTemplate::new(
                StatusCode::NOT_FOUND,
                "Key Result Not Found".to_string(),
            );
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// GET /project/:project_id
pub async fn get_project(
    Extension(pool): Extension<PgPool>,
    extract::Path(project_id): extract::Path<i32>,
) -> axum::response::Response {
    let project_row = sqlx::query!(
        r#"
        SELECT projects.title, objectives.objective_id, objectives.title as objective_title 
        FROM projects 
        LEFT JOIN objectives 
        ON projects.objective_id = objectives.objective_id 
        WHERE projects.project_id = $1;
        "#,
        project_id
    )
    .fetch_one(&pool)
    .await;

    match project_row {
        Ok(project) => {
            let template = templater::InitiativeTemplate::new(
                project.title,
                project.objective_id,
                project.objective_title,
            );
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template = templater::ErrorTemplate::new(
                StatusCode::NOT_FOUND,
                "Key Result Not Found".to_string(),
            );
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// GET /task/:task_id
pub async fn get_task() {}

// GET /measure/:measure_id
pub async fn get_measure() {}

// POST /theme
pub async fn add_theme(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_theme): extract::Json<model::CreateTheme>,
) -> Redirect {
    let _ = sqlx::query(r#"INSERT INTO themes (title) VALUES ($1);"#)
        .bind(create_theme.title)
        .fetch_all(&pool)
        .await;

    Redirect::to("/theme")
}

// POST /objective
pub async fn add_objective(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_objective): extract::Json<model::CreateObjective>,
) -> impl IntoResponse {
    let _ = sqlx::query(r#"INSERT INTO objectives (title, theme_id) VALUES ($1, $2);"#)
        .bind(create_objective.new_title)
        .bind(create_objective.theme_id)
        .fetch_all(&pool)
        .await;
    // let new_title = create_objective.new_title;
    let theme_id = create_objective.theme_id;
    let uri = format!("/theme/{theme_id}/objectives");
    Redirect::to(&uri)
    // format!("Adding objective (title: {new_title}, theme_id: {theme_id})").to_owned()
}

// POST /keyresult
pub async fn add_keyresult(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_keyresult): extract::Json<model::CreateKeyResult>,
) -> Redirect {
    let _ = sqlx::query(r#"INSERT INTO keyresults (title, objective_id) VALUES ($1, $2);"#)
        .bind(create_keyresult.title)
        .fetch_all(&pool)
        .await;

    Redirect::to("/keyresult")
}

// POST /initiative
pub async fn add_initiative(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_initiative): extract::Json<model::CreateInitiative>,
) -> Redirect {
    let _ = sqlx::query(r#"INSERT INTO initiatives (title, objective_id) VALUES ($1, $2);"#)
        .bind(create_initiative.title)
        .fetch_all(&pool)
        .await;

    Redirect::to("/initiative")
}

// POST /project
pub async fn add_project(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_project): extract::Json<model::CreateProject>,
) -> Redirect {
    let _ = sqlx::query(r#"INSERT INTO projects (title, objective_id) VALUES ($1, $2);"#)
        .bind(create_project.title)
        .fetch_all(&pool)
        .await;

    Redirect::to("/project")
}

// POST /task
pub async fn add_task(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_task): extract::Json<model::CreateTask>,
) -> Redirect {
    let _ = sqlx::query(r#"INSERT INTO projects (title, project_id) VALUES ($1, $2);"#)
        .bind(create_task.title)
        .fetch_all(&pool)
        .await;

    Redirect::to("/task")
}

// POST /measure
pub async fn add_measure(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_measurement): extract::Json<model::CreateMeasurement>,
) -> Redirect {
    let _ = sqlx::query(r#"INSERT INTO measurements (title, keyresult_id) VALUES ($1, $2);"#)
        .bind(create_measurement.title)
        .fetch_all(&pool)
        .await;

    Redirect::to("/measurement")
}

// DELETE /theme/:theme_id
pub async fn remove_theme() {}

// DELETE /objective/:objective_id
pub async fn remove_objective() {}

// DELETE /keyresult/:keyresult_id
pub async fn remove_keyresult() {}

// DELETE /initiative/:initiative_id
pub async fn remove_initiative() {}

// DELETE /project/:project_id
pub async fn remove_project() {}

// DELETE /task/:task_id
pub async fn remove_task() {}

// DELETE /measure/:measure_id
pub async fn remove_measure() {}

// PUT /theme/:theme_id
pub async fn update_theme() {}

// PUT /objective/:objective_id
pub async fn update_objective() {}

// PUT /keyresult/:keyresult_id
pub async fn update_keyresult() {}

// PUT /initiative/:initiative_id
pub async fn update_initiative() {}

// PUT /project/:project_id
pub async fn update_project() {}

// PUT /task/:task_id
pub async fn update_task() {}

// PUT /measure/:measure_id
pub async fn update_measure() {}