use crate::{
    apps::system::service::{self, sys_user::UserPageResponse},
    custom_response::{CustomResponse, CustomResponseBuilder, HtmlTemplate},
    pagination::PageParams,
    utils::jwt::AuthBody,
    ResponseResult, Result,
};
//use askama::Template;
use askama::Template;
use axum::{
    extract::Query,
    response::{Html, IntoResponse, Response},
    Form, Json,
};
use db::{
    db_conn,
    system::models::sys_user::{CreateUser, LoginUser, NewUser, UpdateUser},
    DB,
};
use headers::HeaderMap;
use hyper::{StatusCode, header::SET_COOKIE};
#[derive(Template)]
#[template(path = "index.html")]
struct LoginTemplate<'a> {
    name: &'a str,
}
static COOKIE_NAME: &str = "jwt";

pub async fn login_page() -> impl IntoResponse {
    let a = LoginTemplate { name: "world" };
    HtmlTemplate(a)
}

#[derive(Template)]
#[template(path = "workspace.html")]
pub struct WorkSpaceTemplate<'a> {
    name: &'a str,
}

pub async fn login<'a>(
    header: HeaderMap,
    Form(req): Form<LoginUser>,
) -> Result<HtmlTemplate<WorkSpaceTemplate<'a>>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::login(db, req, header).await?;
    //Ok(CustomResponseBuilder::new().body(res).build())
    let a = WorkSpaceTemplate { name: "world" };
    // Build the cookie
    let cookie = format!("{}={}", COOKIE_NAME, res.token);
    let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    Ok(HtmlTemplate(a))
}

pub async fn create(Json(req): Json<NewUser>) -> ResponseResult<CreateUser> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::create_user(db, req).await?;
    Ok(CustomResponseBuilder::new().body(res).build())
}

pub async fn update(Json(req): Json<UpdateUser>) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::update_user(db, req).await?;
    Ok(CustomResponseBuilder::new().body(res).build())
}

pub async fn delete(id: String) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::delete(db, id).await?;
    Ok(CustomResponseBuilder::new().body(res).build())
}

pub async fn list(Query(request): Query<PageParams>) -> ResponseResult<UserPageResponse> {
    let res = service::sys_user::page(request).await.unwrap();
    Ok(CustomResponseBuilder::new().body(res).build())
}
