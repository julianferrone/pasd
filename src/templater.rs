use crate::model;
use askama;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

// THEME TEMPLATES
#[derive(askama::Template)]
#[template(path = "theme.html")]
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
#[template(path = "all_themes.html")]
pub struct AllThemesTemplate {
    pub themes: Option<Vec<model::Theme>>,
}

impl AllThemesTemplate {
    pub fn new(themes: Option<Vec<model::Theme>>) -> AllThemesTemplate {
        AllThemesTemplate { themes: themes }
    }
}

// OBJECTIVE TEMPLATES
#[derive(askama::Template)]
#[template(path = "objective.html")]
pub struct ObjectiveTemplate {
    pub title: String,
    pub theme_id: i32,
    pub theme_title: String,
    pub keyresults: Option<Vec<model::KeyResult>>,
    pub initiatives: Option<Vec<model::Initiative>>,
    pub projects: Option<Vec<model::Project>>,
}

impl ObjectiveTemplate {
    pub fn new(
        title: String,
        theme_id: i32,
        theme_title: String,
        keyresults: Option<Vec<model::KeyResult>>,
        initiatives: Option<Vec<model::Initiative>>,
        projects: Option<Vec<model::Project>>,
    ) -> ObjectiveTemplate {
        ObjectiveTemplate {
            title: title,
            theme_id: theme_id,
            theme_title: theme_title,
            keyresults: keyresults,
            initiatives: initiatives,
            projects: projects,
        }
    }
}

#[derive(askama::Template)]
#[template(path = "list_objectives.html")]
pub struct ListObjectivesTemplate {
    pub objectives: Option<Vec<model::Objective>>,
}

impl ListObjectivesTemplate {
    pub fn new(themes: Option<Vec<model::Objective>>) -> ListObjectivesTemplate {
        ListObjectivesTemplate { objectives: themes }
    }
}

#[derive(askama::Template)]
#[template(path = "all_objectives.html")]
pub struct AllObjectivesTemplate {
    pub objectives: Option<Vec<model::Objective>>,
}

impl AllObjectivesTemplate {
    pub fn new(themes: Option<Vec<model::Objective>>) -> AllObjectivesTemplate {
        AllObjectivesTemplate { objectives: themes }
    }
}

// KEY RESULT TEMPLATES
#[derive(askama::Template)]
#[template(path = "keyresult.html")]
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
#[template(path = "all_keyresults.html")]
pub struct AllKeyResultsTemplate {
    pub keyresults: Option<Vec<model::KeyResult>>,
}

impl AllKeyResultsTemplate {
    pub fn new(keyresults: Option<Vec<model::KeyResult>>) -> AllKeyResultsTemplate {
        AllKeyResultsTemplate { keyresults }
    }
}

// INITIATIVE TEMPLATES
#[derive(askama::Template)]
#[template(path = "initiative.html")]
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
#[template(path = "all_initiatives.html")]
pub struct AllInitiativesTemplate {
    pub initiatives: Option<Vec<model::Initiative>>,
}

impl AllInitiativesTemplate {
    pub fn new(initiatives: Option<Vec<model::Initiative>>) -> AllInitiativesTemplate {
        AllInitiativesTemplate { initiatives }
    }
}

// PROJECT TEMPLATES
#[derive(askama::Template)]
#[template(path = "project.html")]
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
#[template(path = "all_projects.html")]
pub struct AllProjectsTemplate {
    pub projects: Option<Vec<model::Project>>,
}

impl AllProjectsTemplate {
    pub fn new(projects: Option<Vec<model::Project>>) -> AllProjectsTemplate {
        AllProjectsTemplate { projects }
    }
}

#[derive(askama::Template)]
#[template(path = "error.html")]
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
