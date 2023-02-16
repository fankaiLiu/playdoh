use askama::Template;
use axum::extract::Query;
use db::{runtime::models::function_log::FnLog, DB, db_conn};
use crate::Result;
use crate::{pagination::{PageTurnResponse, PageParams}, custom_response::HtmlTemplate, apps::CONTEXT};

#[derive(Template)]
#[template(path = "runtime/function_log.html")]
pub struct FnLogListTemplate {
    data: PageTurnResponse<FnLog>,
}

pub async fn list<'a>(Query(request): Query<PageParams>) -> Result<HtmlTemplate<FnLogListTemplate>> {
   let db = DB.get_or_init(db_conn).await;
   let res = CONTEXT.runtime_function_log.page_function_log(db, request).await?;
   //Ok(CustomResponseBuilder::new().body(res).build())
   let a = FnLogListTemplate { data:res };
   Ok(HtmlTemplate(a))
}