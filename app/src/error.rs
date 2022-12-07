use std::borrow::Cow;
use std::collections::HashMap;

use axum::http::header::WWW_AUTHENTICATE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use sqlx::error::DatabaseError;
use sqlx::types::uuid;
use tokio::task::JoinError;
use tracing::log;

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
    SerderError(#[from] serde_json::Error),
    #[error("{0}")]
    BadRequest(#[from] BadRequest),
    #[error("an error occurred with the database,{0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("an error occurred with the uuid,{0}")]
    Uuid(#[from] uuid::Error),
    #[error("{0}an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized,

    /// Return `403 Forbidden`
    #[error("user may not perform that action")]
    Forbidden,

    /// Return `422 Unprocessable Entity`
    ///
    /// This also serializes the `errors` map to JSON to satisfy the requirement for
    /// `422 Unprocessable Entity` errors in the Realworld spec:
    /// https://realworld-docs.netlify.app/docs/specs/backend-specs/error-handling
    ///
    /// For a good API, the other status codes should also ideally map to some sort of JSON body
    /// that the frontend can deal with, but I do admit sometimes I've just gotten lazy and
    /// returned a plain error message if there were few enough error modes for a route
    /// that the frontend could infer the error from the status code alone.
    #[error("error in the request body")]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },
}

impl Error {
    fn get_codes(&self) -> (StatusCode, u16) {
        let error = self.clone();
        match *error {
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
            Error::Sqlx(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5002),
            Error::Anyhow(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5003),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, 40003),
            Error::Forbidden => (StatusCode::FORBIDDEN, 40003),
            Error::UnprocessableEntity { .. } => (StatusCode::UNPROCESSABLE_ENTITY, 40003),
            Error::Uuid(_) => (StatusCode::BAD_REQUEST, 40003),
            Error::SerderError(_) => (StatusCode::BAD_REQUEST, 40003),
        }
    }
    /// Convenient constructor for `Error::UnprocessableEntity`.
    ///
    /// Multiple for the same key are collected into a list for that key.
    ///
    /// Try "Go to Usage" in an IDE for examples.
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();

        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        Self::UnprocessableEntity { errors: error_map }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::UnprocessableEntity { errors } => {
                #[derive(serde::Serialize)]
                struct Errors {
                    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
                }

                return (StatusCode::UNPROCESSABLE_ENTITY, Json(Errors { errors })).into_response();
            }
            Self::Unauthorized => {
                return (
                    self.get_codes().0,
                    // Include the `WWW-Authenticate` challenge required in the specification
                    // for the `401 Unauthorized` response code:
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401
                    //
                    // The Realworld spec does not specify this:
                    // https://realworld-docs.netlify.app/docs/specs/backend-specs/error-handling
                    //
                    // However, at Launchbadge we try to adhere to web standards wherever possible,
                    // if nothing else than to try to act as a vanguard of sanity on the web.
                    [(WWW_AUTHENTICATE, HeaderValue::from_static("Token"))]
                        .into_iter()
                        .collect::<HeaderMap>(),
                    self.to_string(),
                )
                    .into_response();
            }

            Self::Sqlx(ref e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP request by `TraceLayer`.
                log::error!("SQLx error: {:?}", e);
            }

            Self::Anyhow(ref e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP request by `TraceLayer`.
                log::error!("Generic error: {:?}", e);
            }

            // Other errors get mapped normally.
            _ => (),
        }
        let (status_code, code) = self.get_codes();
        (
            status_code,
            Json(json!({ "code": code, "message": self.to_string() })),
        )
            .into_response()
    }
}

// impl IntoResponse for Error {
//     // fn into_response(self) -> Response {
//     //     let (status_code, code) = self.get_codes();

//     //     let message = self.to_string();
//     //     let body = Json(json!({ "code": code, "message": message }));

//     //     (status_code, body).into_response()
//     // }
//     type Body = Full<Bytes>;
//     type BodyError = <Full<Bytes> as HttpBody>::Error;
//     fn into_response(self) -> Response<Self::Body> {
//         match self {
//             Self::UnprocessableEntity { errors } => {
//                 #[derive(serde::Serialize)]
//                 struct Errors {
//                     errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
//                 }

//                 return (StatusCode::UNPROCESSABLE_ENTITY, Json(Errors { errors })).into_response();
//             }
//             Self::Unauthorized => {
//                 return (
//                     self.status_code(),
//                     // Include the `WWW-Authenticate` challenge required in the specification
//                     // for the `401 Unauthorized` response code:
//                     // https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401
//                     //
//                     // The Realworld spec does not specify this:
//                     // https://realworld-docs.netlify.app/docs/specs/backend-specs/error-handling
//                     //
//                     // However, at Launchbadge we try to adhere to web standards wherever possible,
//                     // if nothing else than to try to act as a vanguard of sanity on the web.
//                     [(WWW_AUTHENTICATE, HeaderValue::from_static("Token"))]
//                         .into_iter()
//                         .collect::<HeaderMap>(),
//                     self.to_string(),
//                 )
//                     .into_response();
//             }

//             Self::Sqlx(ref e) => {
//                 // TODO: we probably want to use `tracing` instead
//                 // so that this gets linked to the HTTP request by `TraceLayer`.
//                 log::error!("SQLx error: {:?}", e);
//             }

//             Self::Anyhow(ref e) => {
//                 // TODO: we probably want to use `tracing` instead
//                 // so that this gets linked to the HTTP request by `TraceLayer`.
//                 log::error!("Generic error: {:?}", e);
//             }

//             // Other errors get mapped normally.
//             _ => (),
//         }

//         (self.get_codes().0, self.to_string()).into_response()
//     }
// }

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
