use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use tokio::task::JoinError;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    Authenticate(#[from] AuthenticateError),
    #[error("{0}")]
    RunSyncTask(#[from] JoinError),
    #[error("{0}")]
    NotFound(#[from] NotFound),
    #[error("{0}")]
    BadRequest(#[from] BadRequest),
}

impl Error {
    fn get_codes(&self) -> (StatusCode, u16) {
        match *self {
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, 40003),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, 40003),

            Error::Authenticate(AuthenticateError::WrongCredentials) => {
                (StatusCode::UNAUTHORIZED, 40003)
            }
            Error::Authenticate(AuthenticateError::InvalidToken) => {
                (StatusCode::UNAUTHORIZED, 40003)
            }
            Error::Authenticate(AuthenticateError::Locked) => (StatusCode::LOCKED, 40003),

            // 5XX Errors
            Error::Authenticate(AuthenticateError::TokenCreation) => {
                (StatusCode::INTERNAL_SERVER_ERROR, 5001)
            }
            Error::RunSyncTask(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5009),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, code) = self.get_codes();
        let message = self.to_string();
        let body = Json(json!({ "code": code, "message": message }));

        (status_code, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AuthenticateError {
    #[error("Wrong authentication credentials")]
    WrongCredentials,
    #[error("Failed to create authentication token")]
    TokenCreation,
    #[error("Invalid authentication credentials")]
    InvalidToken,
    #[error("User is locked")]
    Locked,
}

#[derive(thiserror::Error, Debug)]
#[error("Bad request. Field: {field}, message: {message}")]
pub struct BadRequest {
    pub field: String,
    pub message: String,
}

impl BadRequest {
    pub fn new(field: String, message: String) -> Self {
        BadRequest { field, message }
    }

    // TODO: Implement a proper empty Bad Request error
    pub fn empty() -> Self {
        BadRequest {
            field: String::new(),
            message: String::new(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Not found")]
pub struct NotFound {
    resource: String,
    message: String,
}

impl NotFound {
    pub fn new(resource: String) -> Self {
        NotFound {
            resource: resource.clone(),
            message: format!("{} not found", resource),
        }
    }
}
