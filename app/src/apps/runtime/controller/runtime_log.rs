use askama::Template;
use axum::extract::{Query, Path};
use db::{runtime::models::function_log::FnLog, DB, db_conn};
use uuid::Uuid;
use crate::Result;
use crate::{pagination::{PageTurnResponse, PageParams}, custom_response::HtmlTemplate, apps::CONTEXT};

#[derive(Template)]
#[template(path = "runtime/function_log.html")]
pub struct FnLogListTemplate {
    data: PageTurnResponse<FnLog>,
}

pub async fn list<'a>(Path((id)): Path<(String)>,Query((request)): Query<(PageParams)>) -> Result<HtmlTemplate<FnLogListTemplate>> {
   let db = DB.get_or_init(db_conn).await;
   let id = Uuid::parse_str(&id)?;
   let res = CONTEXT.runtime_function_log.page_function_log(db,&id, request).await?;
   //Ok(CustomResponseBuilder::new().body(res).build())
   let a = FnLogListTemplate { data:res };
   Ok(HtmlTemplate(a))
}