use crate::model;
use askama;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

// ROOT TEMPLATES
#[derive(askama::Template)]
#[template(path = "page/root.html")]
pub struct RootTemplate {
    pub themes: Option<Vec<model::Theme>>,
}

impl RootTemplate {
    pub fn new(themes: Option<Vec<model::Theme>>) -> RootTemplate {
        RootTemplate { themes }
    }
}

// THEME TEMPLATES
#[derive(askama::Template)]
#[template(path = "page/theme.html")]
pub struct ThemeTemplate {
    pub title: String,
    pub theme_id: i32,
    pub theme_status: model::Status,
    pub objectives: Option<Vec<model::Objective>>,
}

impl ThemeTemplate {
    pub fn new(
        title: String,
        theme_id: i32,
        theme_status: model::Status,
        objectives: Option<Vec<model::Objective>>,
    ) -> ThemeTemplate {
        ThemeTemplate {
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

// OBJECTIVE TEMPLATES
#[derive(askama::Template)]
#[template(path = "page/objective.html")]
pub struct ObjectiveTemplate {
    pub title: String,
    pub objective_id: i32,
    pub theme_id: i32,
    pub theme_title: String,
    pub keyresults: Option<Vec<model::KeyResult>>,
    pub initiatives: Option<Vec<model::Initiative>>,
    pub projects: Option<Vec<model::Project>>,
}

impl ObjectiveTemplate {
    pub fn new(
        title: String,
        objective_id: i32,
        theme_id: i32,
        theme_title: String,
        keyresults: Option<Vec<model::KeyResult>>,
        initiatives: Option<Vec<model::Initiative>>,
        projects: Option<Vec<model::Project>>,
    ) -> ObjectiveTemplate {
        ObjectiveTemplate {
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
    pub fn new(objectives: Option<Vec<model::Objective>>, theme_id: i32) -> TableObjectivesTemplate {
        TableObjectivesTemplate {
            objectives,
            theme_id,
        }
    }
}

// KEY RESULT TEMPLATES
#[derive(askama::Template)]
#[template(path = "page/keyresult.html")]
pub struct KeyResultTemplate {
    pub title: String,
    pub objective_id: i32,
    pub objective_title: String,
    pub measurements: Option<Vec<model::Measurement>>,
}

impl KeyResultTemplate {
    pub fn new(
        title: String,
        objective_id: i32,
        objective_title: String,
        measurements: Option<Vec<model::Measurement>>,
    ) -> KeyResultTemplate {
        KeyResultTemplate {
            title,
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

// INITIATIVE TEMPLATES
#[derive(askama::Template)]
#[template(path = "page/initiative.html")]
pub struct InitiativeTemplate {
    pub title: String,
    pub objective_id: i32,
    pub objective_title: String,
}

impl InitiativeTemplate {
    pub fn new(title: String, objective_id: i32, objective_title: String) -> InitiativeTemplate {
        InitiativeTemplate {
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

// PROJECT TEMPLATES
#[derive(askama::Template)]
#[template(path = "page/project.html")]
pub struct ProjectTemplate {
    pub title: String,
    pub objective_id: i32,
    pub objective_title: String,
    pub tasks: Option<Vec<model::Task>>,
}

impl ProjectTemplate {
    pub fn new(
        title: String,
        objective_id: i32,
        objective_title: String,
        tasks: Option<Vec<model::Task>>,
    ) -> ProjectTemplate {
        ProjectTemplate {
            title,
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
