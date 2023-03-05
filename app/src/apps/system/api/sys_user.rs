use std::{future::Future, pin::Pin};

use crate::{
    apps::system::service::{self, sys_user::UserPageResponse},
    custom_response::{CustomResponse, CustomResponseBuilder, HtmlTemplate},
    pagination::PageParams,
    utils::jwt::AuthBody,
    ResponseResult, Result,
};
use askama::Template;
use axum::{
    extract::Query,
    response::{Html, IntoResponse, Redirect, Response},
    Form, Json,
};
use db::{
    db_conn,
    system::models::sys_user::{CreateUser, LoginUser, NewUser, UpdateUser},
    DB,
};
use headers::HeaderMap;
use tower_cookies::{Cookie, Cookies};
#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a> {
    name: &'a str,
}
static COOKIE_NAME: &str = "token";

pub async fn login_check(cookies: Cookies) -> Response {
    //TODO: if cookies is expired, return login_page
    if cookies.get(COOKIE_NAME).is_some() {
        return Redirect::to("/system").into_response();
    }
    let a = LoginTemplate { name: "world" };
    return HtmlTemplate(a).into_response();
}

#[derive(Template)]
#[template(path = "workspace.html")]
pub struct WorkSpaceTemplate<'a> {
    name: &'a str,
}

pub async fn login<'a>(
    header: HeaderMap,
    cookies: Cookies,
    Form(req): Form<LoginUser>,
) -> Result<HtmlTemplate<WorkSpaceTemplate<'a>>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::login(cookies, db, req, header).await?;
    //Ok(CustomResponseBuilder::new().body(res).build())
    let a = WorkSpaceTemplate { name: "world" };
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
