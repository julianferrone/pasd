use crate::{errors::CustomError, model, templater};
use axum::{
    extract,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension,
};
use sqlx::PgPool;

// GET /
pub async fn get_root(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from themes".to_string();

    let themes = sqlx::query_as::<_, model::Theme>(&sql)
        .fetch_all(&pool)
        .await;

    let template = templater::RootTemplate::new(themes.ok().clone());
    templater::HtmlTemplate(template).into_response()
}

// macro_rules! get_all_from_table{
//     ($table_name:expr, $struct:item)=>{
//         pub async fn get_all_$table_name(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
//             let sql = format!("SELECT * from {table_name}").to_string();
//             let $table_name = sql::query_as::<_,
//         }
//     }
// }
// get_all_from_table!("themes".to_string(), model::Theme);

// GET /theme
pub async fn get_root_themes(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * from themes".to_string();

    let themes = sqlx::query_as::<_, model::Theme>(&sql)
        .fetch_all(&pool)
        .await;

    let template = templater::TableThemesTemplate::new(themes.ok().clone());
    templater::HtmlTemplate(template).into_response()
}

// GET /theme/:theme_id
pub async fn get_theme(
    Extension(pool): Extension<PgPool>,
    extract::Path(theme_id): extract::Path<i32>,
) -> axum::response::Response {
    let theme_row: Result<model::Theme, sqlx::Error> =
        sqlx::query_as(r#"SELECT * from themes where theme_id = $1"#)
            .bind(&theme_id)
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

// GET /theme/:theme_id/row
pub async fn get_theme_row(
    Extension(pool): Extension<PgPool>,
    extract::Path(theme_id): extract::Path<i32>,
) -> axum::response::Response {
    let theme_row = sqlx::query_as(r#"SELECT * from themes where theme_id = $1"#)
        .bind(&theme_id)
        .fetch_one(&pool)
        .await;

    match theme_row {
        Ok(theme) => {
            let template = templater::RowThemeTemplate::new(theme);
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template =
                templater::ErrorTemplate::new(StatusCode::NOT_FOUND, "Theme Not Found".to_string());
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// GET /theme/:theme_id/form
pub async fn get_theme_form(
    Extension(pool): Extension<PgPool>,
    extract::Path(theme_id): extract::Path<i32>,
) -> axum::response::Response {
    let theme_row = sqlx::query_as(r#"SELECT * from themes where theme_id = $1"#)
        .bind(&theme_id)
        .fetch_one(&pool)
        .await;

    match theme_row {
        Ok(theme) => {
            let template = templater::EditRowThemeTemplate::new(theme);
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
    let template = templater::TableObjectivesTemplate::new(objectives.clone(), theme_id);
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
                objective_id,
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

// GET /objective/:objective_id/row
pub async fn get_objective_row(
    Extension(pool): Extension<PgPool>,
    extract::Path(objective_id): extract::Path<i32>,
) -> axum::response::Response {
    let objective_row = sqlx::query_as(r#"SELECT * from objectives where objective_id = $1"#)
        .bind(&objective_id)
        .fetch_one(&pool)
        .await;

    match objective_row {
        Ok(objective) => {
            let template = templater::RowObjectiveTemplate::new(objective);
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

// GET /objective/:objective_id/form
pub async fn get_objective_form(
    Extension(pool): Extension<PgPool>,
    extract::Path(objective_id): extract::Path<i32>,
) -> axum::response::Response {
    let objective_row = sqlx::query_as(r#"SELECT * from objectives where objective_id = $1"#)
        .bind(&objective_id)
        .fetch_one(&pool)
        .await;

    match objective_row {
        Ok(objective) => {
            let template = templater::EditRowObjectiveTemplate::new(objective);
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

// GET /objective/:objective_id/keyresults
pub async fn get_objective_keyresults(
    Extension(pool): Extension<PgPool>,
    extract::Path(objective_id): extract::Path<i32>,
) -> axum::response::Response {
    let keyresults: Option<Vec<model::KeyResult>> = sqlx::query_as(
        r#"SELECT *
        FROM keyresults
        where keyresults.objective_id = $1
        ORDER BY keyresults.keyresult_id;"#,
    )
    .bind(&objective_id)
    .fetch_all(&pool)
    .await
    .ok();
    let template = templater::TableKeyResultsTemplate::new(keyresults.clone(), objective_id);
    templater::HtmlTemplate(template).into_response()
}

// GET /objective/:objective_id/initiatives
pub async fn get_objective_initiatives(
    Extension(pool): Extension<PgPool>,
    extract::Path(objective_id): extract::Path<i32>,
) -> axum::response::Response {
    let initiatives: Option<Vec<model::Initiative>> = sqlx::query_as(
        r#"SELECT *
        FROM initiatives
        where initiatives.objective_id = $1
        ORDER BY initiatives.initiative_id;"#,
    )
    .bind(&objective_id)
    .fetch_all(&pool)
    .await
    .ok();
    let template = templater::TableInitiativesTemplate::new(initiatives.clone(), objective_id);
    templater::HtmlTemplate(template).into_response()
}

// GET /objective/:objective_id/projects
pub async fn get_objective_projects(
    Extension(pool): Extension<PgPool>,
    extract::Path(objective_id): extract::Path<i32>,
) -> axum::response::Response {
    let projects: Option<Vec<model::Project>> = sqlx::query_as(
        r#"SELECT *
        FROM projects
        where projects.objective_id = $1
        ORDER BY projects.project_id;"#,
    )
    .bind(&objective_id)
    .fetch_all(&pool)
    .await
    .ok();
    let template = templater::TableProjectsTemplate::new(projects.clone(), objective_id);
    templater::HtmlTemplate(template).into_response()
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
                keyresult_id,
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

// GET /keyresult/:keyresult_id/row
pub async fn get_keyresult_row(
    Extension(pool): Extension<PgPool>,
    extract::Path(keyresult_id): extract::Path<i32>,
) -> axum::response::Response {
    let keyresult_row = sqlx::query_as(r#"SELECT * from keyresults where keyresult_id = $1"#)
        .bind(&keyresult_id)
        .fetch_one(&pool)
        .await;

    match keyresult_row {
        Ok(keyresult) => {
            let template = templater::RowKeyResultTemplate::new(keyresult);
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

// GET /keyresult/:keyresult_id/form
pub async fn get_keyresult_form(
    Extension(pool): Extension<PgPool>,
    extract::Path(keyresult_id): extract::Path<i32>,
) -> axum::response::Response {
    let keyresult_row = sqlx::query_as(r#"SELECT * from keyresults where keyresult_id = $1"#)
        .bind(&keyresult_id)
        .fetch_one(&pool)
        .await;

    match keyresult_row {
        Ok(keyresult) => {
            let template = templater::EditRowKeyResultTemplate::new(keyresult);
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

// GET /keyresult/:keyresult_id/measurements
pub async fn get_keyresult_measurements(
    Extension(pool): Extension<PgPool>,
    extract::Path(keyresult_id): extract::Path<i32>,
) -> axum::response::Response {
    let measurements: Option<Vec<model::Measurement>> = sqlx::query_as(
        r#"SELECT *
        FROM measurements
        where measurements.keyresult_id = $1
        ORDER BY measurements.project_id;"#,
    )
    .bind(&keyresult_id)
    .fetch_all(&pool)
    .await
    .ok();
    let template = templater::TableMeasurementsTemplate::new(measurements.clone(), keyresult_id);
    templater::HtmlTemplate(template).into_response()
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

// GET /initiative/:initiative_id/row
pub async fn get_initiative_row(
    Extension(pool): Extension<PgPool>,
    extract::Path(initiative_id): extract::Path<i32>,
) -> axum::response::Response {
    let initiative_row = sqlx::query_as(r#"SELECT * from initiatives where initiative_id = $1"#)
        .bind(&initiative_id)
        .fetch_one(&pool)
        .await;

    match initiative_row {
        Ok(initiative) => {
            let template = templater::RowInitiativeTemplate::new(initiative);
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template = templater::ErrorTemplate::new(
                StatusCode::NOT_FOUND,
                "Initiative Not Found".to_string(),
            );
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// GET /initiative/:initiative_id/form
pub async fn get_initiative_form(
    Extension(pool): Extension<PgPool>,
    extract::Path(initiative_id): extract::Path<i32>,
) -> axum::response::Response {
    let initiative_row = sqlx::query_as(r#"SELECT * from initiatives where initiative_id = $1"#)
        .bind(&initiative_id)
        .fetch_one(&pool)
        .await;

    match initiative_row {
        Ok(initiative) => {
            let template = templater::EditRowInitiativeTemplate::new(initiative);
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template = templater::ErrorTemplate::new(
                StatusCode::NOT_FOUND,
                "Initiative Not Found".to_string(),
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
            let tasks: Option<Vec<model::Task>> = sqlx::query_as(
                r#"SELECT *
                FROM tasks
                WHERE tasks.project_id = $1
                ORDER BY tasks.task_id;"#,
            )
            .bind(&project_id)
            .fetch_all(&pool)
            .await
            .ok();

            let template = templater::ProjectTemplate::new(
                project.title,
                project_id,
                project.objective_id,
                project.objective_title,
                tasks,
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

// GET /project/:project_id/row
pub async fn get_project_row(
    Extension(pool): Extension<PgPool>,
    extract::Path(project_id): extract::Path<i32>,
) -> axum::response::Response {
    let project_row = sqlx::query_as(r#"SELECT * from projects where project_id = $1"#)
        .bind(&project_id)
        .fetch_one(&pool)
        .await;

    match project_row {
        Ok(project) => {
            let template = templater::RowProjectTemplate::new(project);
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template = templater::ErrorTemplate::new(
                StatusCode::NOT_FOUND,
                "Project Not Found".to_string(),
            );
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// GET /project/:project_id/form
pub async fn get_project_form(
    Extension(pool): Extension<PgPool>,
    extract::Path(project_id): extract::Path<i32>,
) -> axum::response::Response {
    let project_row = sqlx::query_as(r#"SELECT * from projects where project_id = $1"#)
        .bind(&project_id)
        .fetch_one(&pool)
        .await;

    match project_row {
        Ok(project) => {
            let template = templater::EditRowProjectTemplate::new(project);
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template = templater::ErrorTemplate::new(
                StatusCode::NOT_FOUND,
                "Project Not Found".to_string(),
            );
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// GET /project/:project_id/tasks
pub async fn get_project_tasks(
    Extension(pool): Extension<PgPool>,
    extract::Path(project_id): extract::Path<i32>,
) -> axum::response::Response {
    let tasks: Option<Vec<model::Task>> = sqlx::query_as(
        r#"SELECT *
        FROM tasks
        where tasks.project_id = $1
        ORDER BY tasks.task_id;"#,
    )
    .bind(&project_id)
    .fetch_all(&pool)
    .await
    .ok();
    let template = templater::TableTasksTemplate::new(tasks.clone(), project_id);
    templater::HtmlTemplate(template).into_response()
}

// GET /task/:task_id/row
pub async fn get_task_row(
    Extension(pool): Extension<PgPool>,
    extract::Path(task_id): extract::Path<i32>,
) -> axum::response::Response {
    let task_row = sqlx::query_as(r#"SELECT * from tasks where task_id = $1"#)
        .bind(&task_id)
        .fetch_one(&pool)
        .await;

    match task_row {
        Ok(task) => {
            let template = templater::RowTaskTemplate::new(task);
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template =
                templater::ErrorTemplate::new(StatusCode::NOT_FOUND, "Task Not Found".to_string());
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// GET /task/:task_id/form
pub async fn get_task_form(
    Extension(pool): Extension<PgPool>,
    extract::Path(task_id): extract::Path<i32>,
) -> axum::response::Response {
    let task_row = sqlx::query_as(r#"SELECT * from tasks where task_id = $1"#)
        .bind(&task_id)
        .fetch_one(&pool)
        .await;

    match task_row {
        Ok(task) => {
            let template = templater::EditRowTaskTemplate::new(task);
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template =
                templater::ErrorTemplate::new(StatusCode::NOT_FOUND, "Task Not Found".to_string());
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// GET /measure/:measurement_id/row
pub async fn get_measure_row(
    Extension(pool): Extension<PgPool>,
    extract::Path(measurement_id): extract::Path<i32>,
) -> axum::response::Response {
    let measurement_row = sqlx::query_as(r#"SELECT * from measurements where measurement_id = $1"#)
        .bind(&measurement_id)
        .fetch_one(&pool)
        .await;

    match measurement_row {
        Ok(measurement) => {
            let template = templater::RowMeasurementTemplate::new(measurement);
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template = templater::ErrorTemplate::new(
                StatusCode::NOT_FOUND,
                "Measurement Not Found".to_string(),
            );
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// GET /measure/:measurement_id/form
pub async fn get_measure_form(
    Extension(pool): Extension<PgPool>,
    extract::Path(measurement_id): extract::Path<i32>,
) -> axum::response::Response {
    let measurement_row = sqlx::query_as(r#"SELECT * from measurements where measurement_id = $1"#)
        .bind(&measurement_id)
        .fetch_one(&pool)
        .await;

    match measurement_row {
        Ok(measurement) => {
            let template = templater::EditRowMeasurementTemplate::new(measurement);
            templater::HtmlTemplate(template).into_response()
        }
        Err(_) => {
            let template = templater::ErrorTemplate::new(
                StatusCode::NOT_FOUND,
                "Measurement Not Found".to_string(),
            );
            templater::HtmlTemplate(template).into_response()
        }
    }
}

// For non-existent routes
pub async fn get_error_404_page() -> impl IntoResponse {
    let (error_code, error_message) = CustomError::BadRequest.get_error_message();
    let template = templater::ErrorTemplate::new(error_code, error_message);
    templater::HtmlTemplate(template).into_response()
}

// POST /theme
pub async fn add_theme(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_theme): extract::Json<model::CreateTheme>,
) -> Redirect {
    let _ = sqlx::query(r#"INSERT INTO themes (title) VALUES ($1);"#)
        .bind(create_theme.title)
        .fetch_all(&pool)
        .await;

    Redirect::to("/")
}

// POST /objective
pub async fn add_objective(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_objective): extract::Json<model::CreateObjective>,
) -> impl IntoResponse {
    let _ = sqlx::query(r#"INSERT INTO objectives (title, theme_id) VALUES ($1, $2);"#)
        .bind(create_objective.title)
        .bind(create_objective.theme_id)
        .fetch_all(&pool)
        .await;
    let theme_id = create_objective.theme_id;
    let uri = format!("/theme/{theme_id}/objectives");
    Redirect::to(&uri)
}

// POST /keyresult
pub async fn add_keyresult(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_keyresult): extract::Json<model::CreateKeyResult>,
) -> Redirect {
    let _ = sqlx::query(r#"INSERT INTO keyresults (title, objective_id) VALUES ($1, $2);"#)
        .bind(create_keyresult.title)
        .bind(create_keyresult.objective_id)
        .fetch_all(&pool)
        .await;
    let uri = format!(
        "/objective/{objective_id}/keyresults",
        objective_id = create_keyresult.objective_id
    );
    Redirect::to(&uri)
}

// POST /initiative
pub async fn add_initiative(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_initiative): extract::Json<model::CreateInitiative>,
) -> Redirect {
    let _ = sqlx::query(r#"INSERT INTO initiatives (title, objective_id) VALUES ($1, $2);"#)
        .bind(create_initiative.title)
        .bind(create_initiative.objective_id)
        .fetch_all(&pool)
        .await;
    let uri = format!(
        "/objective/{objective_id}/initiatives",
        objective_id = create_initiative.objective_id
    );
    Redirect::to(&uri)
}

// POST /project
pub async fn add_project(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_project): extract::Json<model::CreateProject>,
) -> Redirect {
    let _ = sqlx::query(r#"INSERT INTO projects (title, objective_id) VALUES ($1, $2);"#)
        .bind(create_project.title)
        .bind(create_project.objective_id)
        .fetch_all(&pool)
        .await;
    let uri = format!(
        "/objective/{objective_id}/projects",
        objective_id = create_project.objective_id
    );
    Redirect::to(&uri)
}

// POST /task
pub async fn add_task(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_task): extract::Json<model::CreateTask>,
) -> Redirect {
    let _ = sqlx::query(r#"INSERT INTO tasks (title, project_id) VALUES ($1, $2);"#)
        .bind(create_task.title)
        .bind(create_task.project_id)
        .fetch_all(&pool)
        .await;
    let uri = format!(
        "/project/{project_id}/tasks",
        project_id = create_task.project_id
    );
    Redirect::to(&uri)
}

// POST /measure
pub async fn add_measure(
    Extension(pool): Extension<PgPool>,
    extract::Json(create_measurement): extract::Json<model::CreateMeasurement>,
) -> Redirect {
    let _ = sqlx::query(r#"INSERT INTO measurements (title, keyresult_id) VALUES ($1, $2);"#)
        .bind(create_measurement.title)
        .bind(create_measurement.keyresult_id)
        .fetch_all(&pool)
        .await;
    let uri = format!(
        "/keyresults/{keyresult_id}/measures",
        keyresult_id = create_measurement.keyresult_id
    );
    Redirect::to(&uri)
}

// PUT /theme/:theme_id
pub async fn update_theme(
    Extension(pool): Extension<PgPool>,
    extract::Path(theme_id): extract::Path<i32>,
    extract::Json(update_theme): extract::Json<model::UpdateTheme>,
) -> Redirect {
    let _: Result<model::Theme, sqlx::Error> = sqlx::query_as(
        r#"UPDATE themes SET title=$1, theme_status=$2 WHERE theme_id=$3 RETURNING *"#,
    )
    .bind(update_theme.title)
    .bind(update_theme.status)
    .bind(theme_id)
    .fetch_one(&pool)
    .await;
    // println!("{:#?}", query_result);
    let uri = format!("/theme/{theme_id}/row");
    Redirect::to(&uri)
}

// PUT /objective/:objective_id
pub async fn update_objective(
    Extension(pool): Extension<PgPool>,
    extract::Path(objective_id): extract::Path<i32>,
    extract::Json(update_objective): extract::Json<model::UpdateObjective>,
) -> Redirect {
    let _ = sqlx::query(r#"UPDATE objectives SET title=$1 WHERE objective_id=$2"#)
        .bind(update_objective.title)
        .bind(objective_id)
        .fetch_all(&pool)
        .await;
    let uri = format!("/objective/{objective_id}/row");
    Redirect::to(&uri)
}

// PUT /keyresult/:keyresult_id
pub async fn update_keyresult(
    Extension(pool): Extension<PgPool>,
    extract::Path(keyresult_id): extract::Path<i32>,
    extract::Json(update_keyresult): extract::Json<model::UpdateKeyResult>,
) -> Redirect {
    let _ = sqlx::query(r#"UPDATE keyresults SET title=$1 WHERE keyresult_id=$2"#)
        .bind(update_keyresult.title)
        .bind(keyresult_id)
        .fetch_all(&pool)
        .await;
    let uri = format!("/keyresult/{keyresult_id}/row");
    Redirect::to(&uri)
}

// PUT /initiative/:initiative_id
pub async fn update_initiative(
    Extension(pool): Extension<PgPool>,
    extract::Path(initiative_id): extract::Path<i32>,
    extract::Json(update_initiative): extract::Json<model::UpdateInitiative>,
) -> Redirect {
    let _ = sqlx::query(
        r#"UPDATE initiatives SET title=$1, initiative_status=$2 WHERE initiative_id=$3"#,
    )
    .bind(update_initiative.title)
    .bind(update_initiative.status)
    .bind(initiative_id)
    .fetch_all(&pool)
    .await;
    let uri = format!("/initiative/{initiative_id}/row");
    Redirect::to(&uri)
}

// PUT /project/:project_id
pub async fn update_project(
    Extension(pool): Extension<PgPool>,
    extract::Path(project_id): extract::Path<i32>,
    extract::Json(update_project): extract::Json<model::UpdateProject>,
) -> Redirect {
    let _ = sqlx::query(r#"UPDATE projects SET title=$1, project_status=$2 WHERE project_id=$3"#)
        .bind(update_project.title)
        .bind(update_project.status)
        .bind(project_id)
        .fetch_all(&pool)
        .await;
    let uri = format!("/project/{project_id}/row");
    Redirect::to(&uri)
}

// PUT /task/:task_id
pub async fn update_task(
    Extension(pool): Extension<PgPool>,
    extract::Path(task_id): extract::Path<i32>,
    extract::Json(update_task): extract::Json<model::UpdateTask>,
) -> Redirect {
    let _ = sqlx::query(r#"UPDATE tasks SET title=$1, task_status=$2 WHERE project_id=$3"#)
        .bind(update_task.title)
        .bind(update_task.status)
        .bind(task_id)
        .fetch_all(&pool)
        .await;
    let uri = format!("/task/{task_id}/row");
    Redirect::to(&uri)
}

// PUT /measure/:measure_id
pub async fn update_measure(
    Extension(pool): Extension<PgPool>,
    extract::Path(measure_id): extract::Path<i32>,
    extract::Json(update_measure): extract::Json<model::UpdateMeasurement>,
) -> Redirect {
    let _ = sqlx::query(r#"UPDATE measurements SET title=$1 WHERE measurement_id=$2"#)
        .bind(update_measure.title)
        .bind(measure_id)
        .fetch_all(&pool)
        .await;
    let uri = format!("/measure/{measure_id}/row");
    Redirect::to(&uri)
}

// DELETE /theme/:theme_id
pub async fn remove_theme(
    Extension(pool): Extension<PgPool>,
    extract::Path(theme_id): extract::Path<i32>,
) -> impl IntoResponse {
    let _ = sqlx::query(r#"DELETE FROM themes WHERE theme_id = $1"#)
        .bind(theme_id)
        .fetch_all(&pool)
        .await;
    (StatusCode::OK, "")
}

// DELETE /objective/:objective_id
pub async fn remove_objective(
    Extension(pool): Extension<PgPool>,
    extract::Path(objective_id): extract::Path<i32>,
) -> impl IntoResponse {
    let _ = sqlx::query(r#"DELETE FROM objectives WHERE objective_id = $1"#)
        .bind(objective_id)
        .fetch_all(&pool)
        .await;
    (StatusCode::OK, "")
}

// DELETE /keyresult/:keyresult_id
pub async fn remove_keyresult(
    Extension(pool): Extension<PgPool>,
    extract::Path(keyresult_id): extract::Path<i32>,
) -> impl IntoResponse {
    let _ = sqlx::query(r#"DELETE FROM keyresults WHERE keyresult_id = $1"#)
        .bind(keyresult_id)
        .fetch_all(&pool)
        .await;
    (StatusCode::OK, "")
}

// DELETE /initiative/:initiative_id
pub async fn remove_initiative(
    Extension(pool): Extension<PgPool>,
    extract::Path(initiative_id): extract::Path<i32>,
) -> impl IntoResponse {
    let _ = sqlx::query(r#"DELETE FROM initiatives WHERE initiative_id = $1"#)
        .bind(initiative_id)
        .fetch_all(&pool)
        .await;
    (StatusCode::OK, "")
}

// DELETE /project/:project_id
pub async fn remove_project(
    Extension(pool): Extension<PgPool>,
    extract::Path(project_id): extract::Path<i32>,
) -> impl IntoResponse {
    let _ = sqlx::query(r#"DELETE FROM projects WHERE project_id = $1"#)
        .bind(project_id)
        .fetch_all(&pool)
        .await;
    (StatusCode::OK, "")
}

// DELETE /task/:task_id
pub async fn remove_task(
    Extension(pool): Extension<PgPool>,
    extract::Path(task_id): extract::Path<i32>,
) -> impl IntoResponse {
    let _ = sqlx::query(r#"DELETE FROM tasks WHERE task_id = $1"#)
        .bind(task_id)
        .fetch_all(&pool)
        .await;
    (StatusCode::OK, "")
}

// DELETE /measure/:measure_id
pub async fn remove_measure(
    Extension(pool): Extension<PgPool>,
    extract::Path(measure_id): extract::Path<i32>,
) -> impl IntoResponse {
    let _ = sqlx::query(r#"DELETE FROM measures WHERE measure_id = $1"#)
        .bind(measure_id)
        .fetch_all(&pool)
        .await;
    (StatusCode::OK, "")
}