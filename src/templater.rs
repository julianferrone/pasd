use crate::model;
use askama;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use model::Status;

// ROOT TEMPLATES
#[derive(askama::Template)]
#[template(path = "page/root.html")]
pub struct PageRootTemplate {
    pub themes: Option<Vec<model::Theme>>,
}

impl PageRootTemplate {
    pub fn new(themes: Option<Vec<model::Theme>>) -> PageRootTemplate {
        PageRootTemplate { themes }
    }
}

// THEME TEMPLATES
#[derive(askama::Template)]
#[template(path = "page/theme.html")]
pub struct PageThemeTemplate {
    pub title: String,
    pub theme_id: i32,
    pub theme_status: model::Status,
    pub objectives: Option<Vec<model::Objective>>,
}

impl PageThemeTemplate {
    pub fn new(
        title: String,
        theme_id: i32,
        theme_status: model::Status,
        objectives: Option<Vec<model::Objective>>,
    ) -> PageThemeTemplate {
        PageThemeTemplate {
            title: title,
            theme_id: theme_id,
            theme_status: theme_status,
            objectives: objectives,
        }
    }
}

#[derive(askama::Template)]
#[template(path = "table/themes.html")]
pub struct TableThemesTemplate {
    pub themes: Option<Vec<model::Theme>>,
}

impl TableThemesTemplate {
    pub fn new(themes: Option<Vec<model::Theme>>) -> TableThemesTemplate {
        TableThemesTemplate { themes }
    }
}

#[derive(askama::Template)]
#[template(path = "row/theme.html")]
pub struct RowThemeTemplate {
    pub theme: model::Theme,
}

impl RowThemeTemplate {
    pub fn new(theme: model::Theme) -> RowThemeTemplate {
        RowThemeTemplate { theme }
    }
}

#[derive(askama::Template)]
#[template(path = "form/edit_theme.html")]
pub struct EditRowThemeTemplate {
    pub theme: model::Theme,
}

impl EditRowThemeTemplate {
    pub fn new(theme: model::Theme) -> EditRowThemeTemplate {
        EditRowThemeTemplate { theme }
    }
}

// OBJECTIVE TEMPLATES
#[derive(askama::Template)]
#[template(path = "page/objective.html")]
pub struct PageObjectiveTemplate {
    pub title: String,
    pub objective_id: i32,
    pub theme_id: i32,
    pub theme_title: String,
    pub keyresults: Option<Vec<model::KeyResult>>,
    pub initiatives: Option<Vec<model::Initiative>>,
    pub projects: Option<Vec<model::Project>>,
}

impl PageObjectiveTemplate {
    pub fn new(
        title: String,
        objective_id: i32,
        theme_id: i32,
        theme_title: String,
        keyresults: Option<Vec<model::KeyResult>>,
        initiatives: Option<Vec<model::Initiative>>,
        projects: Option<Vec<model::Project>>,
    ) -> PageObjectiveTemplate {
        PageObjectiveTemplate {
            title,
            objective_id,
            theme_id,
            theme_title,
            keyresults,
            initiatives,
            projects,
        }
    }
}

#[derive(askama::Template)]
#[template(path = "table/objectives.html")]
pub struct TableObjectivesTemplate {
    pub objectives: Option<Vec<model::Objective>>,
    pub theme_id: i32,
}

impl TableObjectivesTemplate {
    pub fn new(
        objectives: Option<Vec<model::Objective>>,
        theme_id: i32,
    ) -> TableObjectivesTemplate {
        TableObjectivesTemplate {
            objectives,
            theme_id,
        }
    }
}

#[derive(askama::Template)]
#[template(path = "row/objective.html")]
pub struct RowObjectiveTemplate {
    pub objective: model::Objective,
}

impl RowObjectiveTemplate {
    pub fn new(objective: model::Objective) -> RowObjectiveTemplate {
        RowObjectiveTemplate { objective }
    }
}

#[derive(askama::Template)]
#[template(path = "form/edit_objective.html")]
pub struct EditRowObjectiveTemplate {
    pub objective: model::Objective,
}

impl EditRowObjectiveTemplate {
    pub fn new(objective: model::Objective) -> EditRowObjectiveTemplate {
        EditRowObjectiveTemplate { objective }
    }
}

// KEY RESULT TEMPLATES
#[derive(askama::Template)]
#[template(path = "page/keyresult.html")]
pub struct PageKeyResultTemplate {
    pub title: String,
    pub keyresult_id: i32,
    pub objective_id: i32,
    pub objective_title: String,
    pub measurements: Option<Vec<model::Measurement>>,
}

impl PageKeyResultTemplate {
    pub fn new(
        title: String,
        objective_id: i32,
        keyresult_id: i32,
        objective_title: String,
        measurements: Option<Vec<model::Measurement>>,
    ) -> PageKeyResultTemplate {
        PageKeyResultTemplate {
            title,
            keyresult_id,
            objective_id,
            objective_title,
            measurements,
        }
    }
}

#[derive(askama::Template)]
#[template(path = "table/keyresults.html")]
pub struct TableKeyResultsTemplate {
    pub keyresults: Option<Vec<model::KeyResult>>,
    pub objective_id: i32,
}

impl TableKeyResultsTemplate {
    pub fn new(
        keyresults: Option<Vec<model::KeyResult>>,
        objective_id: i32,
    ) -> TableKeyResultsTemplate {
        TableKeyResultsTemplate {
            keyresults,
            objective_id,
        }
    }
}

#[derive(askama::Template)]
#[template(path = "row/keyresult.html")]
pub struct RowKeyResultTemplate {
    pub keyresult: model::KeyResult,
}

impl RowKeyResultTemplate {
    pub fn new(keyresult: model::KeyResult) -> RowKeyResultTemplate {
        RowKeyResultTemplate { keyresult }
    }
}

#[derive(askama::Template)]
#[template(path = "form/edit_keyresult.html")]
pub struct EditRowKeyResultTemplate {
    pub keyresult: model::KeyResult,
}

impl EditRowKeyResultTemplate {
    pub fn new(keyresult: model::KeyResult) -> EditRowKeyResultTemplate {
        EditRowKeyResultTemplate { keyresult }
    }
}

// INITIATIVE TEMPLATES
#[derive(askama::Template)]
#[template(path = "page/initiative.html")]
pub struct PageInitiativeTemplate {
    pub title: String,
    pub objective_id: i32,
    pub objective_title: String,
}

impl PageInitiativeTemplate {
    pub fn new(title: String, objective_id: i32, objective_title: String) -> PageInitiativeTemplate {
        PageInitiativeTemplate {
            title,
            objective_id,
            objective_title,
        }
    }
}

#[derive(askama::Template)]
#[template(path = "table/initiatives.html")]
pub struct TableInitiativesTemplate {
    pub initiatives: Option<Vec<model::Initiative>>,
    pub objective_id: i32,
}

impl TableInitiativesTemplate {
    pub fn new(
        initiatives: Option<Vec<model::Initiative>>,
        objective_id: i32,
    ) -> TableInitiativesTemplate {
        TableInitiativesTemplate {
            initiatives,
            objective_id,
        }
    }
}

#[derive(askama::Template)]
#[template(path = "row/initiative.html")]
pub struct RowInitiativeTemplate {
    pub initiative: model::Initiative,
}

impl RowInitiativeTemplate {
    pub fn new(initiative: model::Initiative) -> RowInitiativeTemplate {
        RowInitiativeTemplate { initiative }
    }
}

#[derive(askama::Template)]
#[template(path = "form/edit_initiative.html")]
pub struct EditRowInitiativeTemplate {
    pub initiative: model::Initiative,
}

impl EditRowInitiativeTemplate {
    pub fn new(initiative: model::Initiative) -> EditRowInitiativeTemplate {
        EditRowInitiativeTemplate { initiative }
    }
}

// PROJECT TEMPLATES
#[derive(askama::Template)]
#[template(path = "page/project.html")]
pub struct PageProjectTemplate {
    pub title: String,
    pub project_id: i32,
    pub objective_id: i32,
    pub objective_title: String,
    pub tasks: Option<Vec<model::Task>>,
}

impl PageProjectTemplate {
    pub fn new(
        title: String,
        project_id: i32,
        objective_id: i32,
        objective_title: String,
        tasks: Option<Vec<model::Task>>,
    ) -> PageProjectTemplate {
        PageProjectTemplate {
            title,
            project_id,
            objective_id,
            objective_title,
            tasks,
        }
    }
}

#[derive(askama::Template)]
#[template(path = "table/projects.html")]
pub struct TableProjectsTemplate {
    pub projects: Option<Vec<model::Project>>,
    pub objective_id: i32,
}

impl TableProjectsTemplate {
    pub fn new(projects: Option<Vec<model::Project>>, objective_id: i32) -> TableProjectsTemplate {
        TableProjectsTemplate {
            projects,
            objective_id,
        }
    }
}

#[derive(askama::Template)]
#[template(path = "row/project.html")]
pub struct RowProjectTemplate {
    pub project: model::Project,
}

impl RowProjectTemplate {
    pub fn new(project: model::Project) -> RowProjectTemplate {
        RowProjectTemplate { project }
    }
}

#[derive(askama::Template)]
#[template(path = "form/edit_project.html")]
pub struct EditRowProjectTemplate {
    pub project: model::Project,
}

impl EditRowProjectTemplate {
    pub fn new(project: model::Project) -> EditRowProjectTemplate {
        EditRowProjectTemplate { project }
    }
}

// MEASUREMENT TEMPLATES
#[derive(askama::Template)]
#[template(path = "table/measurements.html")]
pub struct TableMeasurementsTemplate {
    pub measurements: Option<Vec<model::Measurement>>,
    pub keyresult_id: i32,
}

impl TableMeasurementsTemplate {
    pub fn new(
        measurements: Option<Vec<model::Measurement>>,
        keyresult_id: i32,
    ) -> TableMeasurementsTemplate {
        TableMeasurementsTemplate {
            measurements,
            keyresult_id,
        }
    }
}

#[derive(askama::Template)]
#[template(path = "row/measurement.html")]
pub struct RowMeasurementTemplate {
    pub measurement: model::Measurement,
}

impl RowMeasurementTemplate {
    pub fn new(measurement: model::Measurement) -> RowMeasurementTemplate {
        RowMeasurementTemplate { measurement }
    }
}

#[derive(askama::Template)]
#[template(path = "form/edit_measurement.html")]
pub struct EditRowMeasurementTemplate {
    pub measurement: model::Measurement,
}

impl EditRowMeasurementTemplate {
    pub fn new(measurement: model::Measurement) -> EditRowMeasurementTemplate {
        EditRowMeasurementTemplate { measurement }
    }
}

// TASK TEMPLATES
#[derive(askama::Template)]
#[template(path = "table/tasks.html")]
pub struct TableTasksTemplate {
    pub tasks: Option<Vec<model::Task>>,
    pub project_id: i32,
}

impl TableTasksTemplate {
    pub fn new(
        tasks: Option<Vec<model::Task>>,
        project_id: i32,
    ) -> TableTasksTemplate {
        TableTasksTemplate {
            tasks,
            project_id,
        }
    }
}


#[derive(askama::Template)]
#[template(path = "row/task.html")]
pub struct RowTaskTemplate {
    pub task: model::Task,
}

impl RowTaskTemplate {
    pub fn new(task: model::Task) -> RowTaskTemplate {
        RowTaskTemplate { task }
    }
}

#[derive(askama::Template)]
#[template(path = "form/edit_task.html")]
pub struct EditRowTaskTemplate {
    pub task: model::Task,
}

impl EditRowTaskTemplate {
    pub fn new(task: model::Task) -> EditRowTaskTemplate {
        EditRowTaskTemplate { task }
    }
}

// ERROR 404 page
#[derive(askama::Template)]
#[template(path = "page/error.html")]
pub struct ErrorTemplate {
    pub error_code: StatusCode,
    pub error_message: String,
}

impl ErrorTemplate {
    pub fn new(error_code: StatusCode, error_message: String) -> ErrorTemplate {
        ErrorTemplate {
            error_code,
            error_message,
        }
    }
}

// Convert templates into HTML
pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: askama::Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
