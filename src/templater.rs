use crate::model;
use askama;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(askama::Template)]
#[template(path = "theme.html")]
pub struct ThemeTemplate {
    pub title: String,
    // pub theme_id: i32,
    pub theme_status: model::Status,
    pub objectives: Option<Vec<model::Objective>>,
}

impl ThemeTemplate {
    pub fn new(
        title: String,
        theme_status: model::Status,
        objectives: Option<Vec<model::Objective>>,
    ) -> ThemeTemplate {
        ThemeTemplate {
            title: title,
            // theme_id: theme.theme_id,
            theme_status: theme_status,
            objectives: objectives,
        }
    }
}

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
