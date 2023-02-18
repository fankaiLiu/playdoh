use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::controller::{runtime_function, runtime_log};
pub fn runtime_function_api() -> Router {
    Router::new()
        .route("/list", get(runtime_function::list))
        .route("/add", get(runtime_function::add))
        .route("/add", post(runtime_function::careate))
        .route("/:source/:function_id",  get(runtime_function::run))
        .route("/log/:function_id", get(runtime_log::list))
}