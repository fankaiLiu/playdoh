use crate::apps::CONTEXT;
use crate::apps::runtime::service::runtime_function::FnDevPageResponse;
use crate::custom_response::CustomResponseBuilder;
use crate::utils::jwt::Claims;
use crate::{custom_response::HtmlTemplate, pagination::PageParams, ResponseResult};
use askama::Template;
use axum::{extract::Query, Json};
use db::runtime::entities::sys_function_dev::*;
use db::runtime::models::{function_log::Source, sys_function_dev::*};
use db::{db_conn, DB};
use hyper::StatusCode;
use uuid::Uuid;

pub async fn careate(Json(req): Json<AddReq>) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = CONTEXT.runtime_funciton.add_function_dev(db, req).await;
    match res {
        Ok(x) => Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Ok(CustomResponseBuilder::new()
            .body(e.to_string())
            .status_code(StatusCode::INTERNAL_SERVER_ERROR)
            .build()),
    }
}

pub async fn update(
    user: Claims,
    Json(req): Json<UpdateReq>,
) -> ResponseResult<Option<FunctionDev>> {
    let db = DB.get_or_init(db_conn).await;
    let user_id = Uuid::parse_str(&user.id)?;
    let res = CONTEXT
        .runtime_funciton
        .update_function_dev(db, &req, &user_id)
        .await?;
    Ok(CustomResponseBuilder::new().body(res).build())
}

pub async fn delete(id: String) -> ResponseResult<bool> {
    let db = DB.get_or_init(db_conn).await;
    let id= Uuid::parse_str(&id)?;
    let res = CONTEXT.runtime_funciton.delete_function_dev(db, &id).await?;
    Ok(CustomResponseBuilder::new().body(res).build())
 }

pub async fn list(Query(request): Query<PageParams>) -> ResponseResult<FnDevPageResponse> {
    let db = DB.get_or_init(db_conn).await;
    let res = CONTEXT.runtime_funciton.page_function_dev(db, request).await?;
    Ok(CustomResponseBuilder::new().body(res).build())
 }
