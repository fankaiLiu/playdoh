use crate::apps::CONTEXT;
use crate::apps::common::alerts::SuccessAlertWithButtonTemplate;
use crate::apps::runtime::service::runtime_function::FnPageResponse;
use crate::custom_response::CustomResponseBuilder;
use crate::pagination::PageTurnResponse;
use crate::utils::jwt::Claims;
use crate::{custom_response::HtmlTemplate, pagination::PageParams, ResponseResult};
use askama::Template;
use axum::Form;
use axum::extract::Path;
use axum::response::Response;
use axum::{extract::Query, Json};
use db::runtime::entities::function::*;
use db::runtime::models::{function_log::Source, function::*};
use db::{db_conn, DB};
use headers::HeaderMap;
use hyper::StatusCode;
use uuid::Uuid;
use crate::Result;
pub async fn careate(
    user: Claims,
    Form(req): Form<AddReq>) -> ResponseResult<String> {
    dbg!(&req);
    let db = DB.get_or_init(db_conn).await;
    let user_id = Uuid::parse_str(&user.id)?;
    let res = CONTEXT.runtime_funciton.add_function(db, req,&user_id).await;
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
) -> ResponseResult<Option<Function>> {
    let db = DB.get_or_init(db_conn).await;
    let user_id = Uuid::parse_str(&user.id)?;
    let res = CONTEXT
        .runtime_funciton
        .update_function(db, &req, &user_id)
        .await?;
    Ok(CustomResponseBuilder::new().body(res).build())
}

pub async fn delete(id: String) -> ResponseResult<bool> {
    let db = DB.get_or_init(db_conn).await;
    let id= Uuid::parse_str(&id)?;
    let res = CONTEXT.runtime_funciton.delete_function(db, &id).await?;
    Ok(CustomResponseBuilder::new().body(res).build())
 }
 #[derive(Template)]
 #[template(path = "runtime/fuction_list.html")]
 pub struct FunctionListTemplate {
     data: PageTurnResponse<Function>,
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

 pub async fn run(
    user: Claims,
    Path(param): Path<(String,String)>,
) -> Result<HtmlTemplate<SuccessAlertWithButtonTemplate>>{
    if param.0=="dev".to_string() {
        let db = DB.get_or_init(db_conn).await;
        let function_id = Uuid::parse_str(&param.1)?;
        let user_id=Uuid::parse_str(&user.id)?;
        let res = CONTEXT.runtime_funciton.run(db, &function_id,Some(user_id)).await?;
        //change header
        let html = SuccessAlertWithButtonTemplate {msg:res};
        return Ok(HtmlTemplate(html));
    }
   todo!()
 }