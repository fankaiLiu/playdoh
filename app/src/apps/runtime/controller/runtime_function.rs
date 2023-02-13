use crate::apps::CONTEXT;
use crate::apps::runtime::service::runtime_function::FnDevPageResponse;
use crate::custom_response::CustomResponseBuilder;
use crate::pagination::PageTurnResponse;
use crate::utils::jwt::Claims;
use crate::{custom_response::HtmlTemplate, pagination::PageParams, ResponseResult};
use askama::Template;
use axum::Form;
use axum::extract::Path;
use axum::{extract::Query, Json};
use db::runtime::entities::sys_function_dev::*;
use db::runtime::models::{function_log::Source, sys_function_dev::*};
use db::{db_conn, DB};
use hyper::StatusCode;
use uuid::Uuid;
use crate::Result;
pub async fn careate(
    user: Claims,
    Form(req): Form<AddReq>) -> ResponseResult<String> {
    dbg!(&req);
    let db = DB.get_or_init(db_conn).await;
    let user_id = Uuid::parse_str(&user.id)?;
    let res = CONTEXT.runtime_funciton.add_function_dev(db, req,&user_id).await;
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
 #[derive(Template)]
 #[template(path = "runtime/fuction_list.html")]
 pub struct FunctionListTemplate {
     data: PageTurnResponse<FunctionDev>,
 }

pub async fn list<'a>(Query(request): Query<PageParams>) -> Result<HtmlTemplate<FunctionListTemplate>> {
    let db = DB.get_or_init(db_conn).await;
    let res = CONTEXT.runtime_funciton.page_function_dev(db, request).await?;
    //Ok(CustomResponseBuilder::new().body(res).build())
    let a = FunctionListTemplate { data:res };
    Ok(HtmlTemplate(a))
 }
 #[derive(Template)]
 #[template(path = "runtime/fuction_add.html")]
 pub struct FunctionAddTemplate {
 }

 
pub async fn add() -> Result<HtmlTemplate<FunctionAddTemplate>> {
    let a = FunctionAddTemplate {   };
    Ok(HtmlTemplate(a))
 }

 pub async fn run(Path(source): Path<String>,Path(function_id): Path<String>,) -> Result<String> {
    if source=="dev".to_string() {
        let db = DB.get_or_init(db_conn).await;
        let function_id = Uuid::parse_str(&function_id)?;
        //let res = CONTEXT.runtime_funciton.(db, &user_id).await?;
        //return Ok(res);
    }
   todo!()
 }