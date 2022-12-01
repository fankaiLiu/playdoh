use axum::Json;
use db::{db_conn, DB};
use headers::HeaderMap;
use crate::{
    apps::system::service::{
        self,
        sys_user::{CreateUser, LoginUser, NewUser, UpdateUser, UserRequest, UserPageClient, UserPageResponse},
    },
    custom_response::{CustomResponse, CustomResponseBuilder},
    utils::jwt::AuthBody,
    ResponseResult, Result, request_query::Page,
};
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

pub async fn list(Json(request): Json<UserRequest> )-> ResponseResult<UserPageResponse> {
    let client=UserPageClient::new();
    let res=client.page(request).await.unwrap();
    Ok(CustomResponseBuilder::new().body(res).build())
}

pub async fn login(
    header: HeaderMap,
    Json(req): Json<LoginUser>,
) -> Result<CustomResponse<AuthBody>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::login(db, req, header).await?;
    Ok(CustomResponseBuilder::new().body(res).build())
}
