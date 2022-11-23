use axum::{Json};
use db::{db_conn, DB};
use headers::HeaderMap;

use crate::{
    apps::system::service::{
        self,
        sys_user::{LoginUser, NewUser,  UserBody, CreateUser},
    },
    Result, utils::jwt::AuthBody, custom_response::{CustomResponseBuilder, CustomResponse}, ResponseResult,
};
pub async fn create(Json(req): Json<UserBody<NewUser>>) -> ResponseResult<CreateUser> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::create_user(db, req).await?;
    Ok(CustomResponseBuilder::new().body(res).build())
}

pub async fn login(Json(req): Json<UserBody<LoginUser>>,header: HeaderMap) ->  Result<CustomResponse<AuthBody>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::login(db, req,header).await?;
    Ok(CustomResponseBuilder::new().body(res).build())
}
