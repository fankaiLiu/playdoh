use axum::{
    http::header::{self, HeaderName, HeaderValue},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bytes::{BufMut, BytesMut};
use serde::Serialize;
use sqlx::error::DatabaseError;
use tracing::error;

use crate::{error::Error, pagination::Pagination};

#[derive(Debug)]
pub struct CustomResponse<T: Serialize> {
    pub body: Option<T>,
    pub status_code: StatusCode,
    pub pagination: Option<Pagination>,
}

pub struct CustomResponseBuilder<T: Serialize> {
    pub body: Option<T>,
    pub status_code: StatusCode,
    pub pagination: Option<Pagination>,
}

impl<T> Default for CustomResponseBuilder<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            body: None,
            status_code: StatusCode::OK,
            pagination: None,
        }
    }
}

impl<T> CustomResponseBuilder<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn body(mut self, body: T) -> Self {
        self.body = Some(body);
        self
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }

    pub fn build(self) -> CustomResponse<T> {
        CustomResponse {
            body: self.body,
            status_code: self.status_code,
            pagination: self.pagination,
        }
    }
}

impl<T> IntoResponse for CustomResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = match self.body {
            Some(body) => body,
            None => return (self.status_code).into_response(),
        };

        let mut bytes = BytesMut::new().writer();
        if let Err(err) = serde_json::to_writer(&mut bytes, &body) {
            error!("Error serializing response body as JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }

        match self.pagination {
            Some(pagination) => {
                let count = pagination.count.to_string();
                let offset = pagination.offset.to_string();
                let limit = pagination.limit.to_string();
                let headers = [
                    (
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                    ),
                    (
                        HeaderName::from_static("x-pagination-count"),
                        HeaderValue::from_str(&count).unwrap(),
                    ),
                    (
                        HeaderName::from_static("x-pagination-offset"),
                        HeaderValue::from_str(&offset).unwrap(),
                    ),
                    (
                        HeaderName::from_static("x-pagination-limit"),
                        HeaderValue::from_str(&limit).unwrap(),
                    ),
                ];

                let bytes = bytes.into_inner().freeze();
                (self.status_code, headers, bytes).into_response()
            }
            None => {
                let headers = [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                )];

                let bytes = bytes.into_inner().freeze();
                let res_json_string = ResJsonString(String::from_utf8(bytes.to_vec()).unwrap());
                let mut response = (self.status_code, headers, bytes).into_response();
                response.extensions_mut().insert(res_json_string);
                response
            }
        }
    }
}

#[derive(Debug)]
pub struct ResJsonString(pub String);
pub trait ResultExt<T> {
    /// If `self` contains a SQLx database constraint error with the given name,
    /// transform the error.
    ///
    /// Otherwise, the result is passed through unchanged.
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}
