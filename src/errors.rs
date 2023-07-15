use axum::http::StatusCode;

pub enum CustomError {
    BadRequest,
    ThemeNotFound,
    ObjectiveNotFound,
    KeyResultNotFound,
    InitiativeNotFound,
    ProjectNotFound,
    TaskNotFound,
    MeasurementNotFound,
    InternalServerError,
}

impl CustomError {
    pub fn get_error_message(self) -> (StatusCode, String) {
        let (status, error_message) = match self {
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
            Self::ThemeNotFound => (StatusCode::NOT_FOUND, "Theme Not Found"),
            Self::ObjectiveNotFound => (StatusCode::NOT_FOUND, "Objective Not Found"),
            Self::KeyResultNotFound => (StatusCode::NOT_FOUND, "Key Result Not Found"),
            Self::InitiativeNotFound => (StatusCode::NOT_FOUND, "Initiative Not Found"),
            Self::ProjectNotFound => (StatusCode::NOT_FOUND, "Project Not Found"),
            Self::TaskNotFound => (StatusCode::NOT_FOUND, "Task Not Found"),
            Self::MeasurementNotFound => (StatusCode::NOT_FOUND, "Measurement Not Found"),
        };
        (status, error_message.to_owned())
    }
}

// impl IntoResponse for CustomError {
//     fn into_response(self) -> axum::response::Response {
//         let (status, error_message) = match self {
//             Self::InternalServerError => {
//                 (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
//             }
//             Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
//             Self::ThemeNotFound => (StatusCode::NOT_FOUND, "Theme Not Found"),
//             Self::ObjectiveNotFound => (StatusCode::NOT_FOUND, "Objective Not Found"),
//             Self::KeyResultNotFound => (StatusCode::NOT_FOUND, "Key Result Not Found"),
//             Self::InitiativeNotFound => (StatusCode::NOT_FOUND, "Initiative Not Found"),
//             Self::ProjectNotFound => (StatusCode::NOT_FOUND, "Project Not Found"),
//             Self::TaskNotFound => (StatusCode::NOT_FOUND, "Task Not Found"),
//             Self::MeasurementNotFound => (StatusCode::NOT_FOUND, "Measurement Not Found"),
//         };
//         let template = templater::ErrorTemplate::new(status, error_message.to_owned());
//         templater::HtmlTemplate(template).into_response()
//     }
// }

// impl IntoResponse for CustomError {
//     fn into_response(self) -> axum::response::Response {
//         let (status, error_message) = match self {
//             Self::InternalServerError => {
//                 (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
//             }
//             Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
//             Self::ThemeNotFound => (StatusCode::NOT_FOUND, "Theme Not Found"),
//             Self::ObjectiveNotFound => (StatusCode::NOT_FOUND, "Objective Not Found"),
//             Self::KeyResultNotFound => (StatusCode::NOT_FOUND, "Key Result Not Found"),
//             Self::InitiativeNotFound => (StatusCode::NOT_FOUND, "Initiative Not Found"),
//             Self::ProjectNotFound => (StatusCode::NOT_FOUND, "Project Not Found"),
//             Self::TaskNotFound => (StatusCode::NOT_FOUND, "Task Not Found"),
//             Self::MeasurementNotFound => (StatusCode::NOT_FOUND, "Measurement Not Found"),
//         };
//         (status, Json(json!({ "error": error_message }))).into_response()
//     }
// }
