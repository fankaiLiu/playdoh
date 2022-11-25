use axum::{routing::post, Router};
mod sys_user; // 用户管理
pub use sys_user::login;

pub fn system_api() -> Router {
    Router::new().nest("/user", sys_user_api()) // 用户管理模块
}

fn sys_user_api() -> Router {
    Router::new().route("/create", post(sys_user::create)) // 添加用户
}
