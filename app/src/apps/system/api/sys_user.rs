use crate::{
    apps::system::service::{
        self,
        sys_user::{CreateUser, LoginUser, NewUser, UpdateUser, UserPageResponse},
    },
    custom_response::{CustomResponse, CustomResponseBuilder},
    pagination::PageParams,
    utils::jwt::AuthBody,
    ResponseResult, Result,
};
use axum::{
    extract::{Query},
    Json,
};
use db::{db_conn, DB};
use headers::HeaderMap;
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
    Json(req): Json<LoginUser>,
) -> Result<CustomResponse<AuthBody>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::login(db, req, header).await?;
    Ok(CustomResponseBuilder::new().body(res).build())
}
