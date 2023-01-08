use crate::{
    apps::system::service::{
        self, sys_user::UserPageResponse,
    },
    custom_response::{CustomResponse, CustomResponseBuilder},
    pagination::PageParams,
    utils::jwt::AuthBody,
    ResponseResult, Result,
};
//use askama::Template;
use axum::{
    extract::{Query},
    Json, response::{IntoResponse, Response, Html}, Form,
};
use db::{db_conn, DB, system::models::sys_user::{NewUser, CreateUser, LoginUser, UpdateUser}};
use headers::HeaderMap;
use askama::Template;
use hyper::StatusCode;
#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a> {
    name: &'a str,
}

pub async fn login_page() ->impl IntoResponse {
   let a=  LoginTemplate { name: "world" };
   HtmlTemplate(a)
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

pub async fn login(
    header: HeaderMap,
    Form(req): Form<LoginUser>,
) -> Result<CustomResponse<AuthBody>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::login(db, req, header).await?;
    Ok(CustomResponseBuilder::new().body(res).build())
}

struct HtmlTemplate<T>(T);
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}