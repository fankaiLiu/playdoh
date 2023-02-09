use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::controller::runtime_function;
pub fn runtime_function_api() -> Router {
    Router::new()
        .route("/list", get(runtime_function::list))
}