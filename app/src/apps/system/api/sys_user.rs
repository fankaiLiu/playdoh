use axum::Json;
use db::{db_conn, DB};

use crate::{
    apps::system::service::{
        self,
        sys_user::{LoginUser, NewUser, User, UserBody},
    },
    Result, utils::jwt::AuthBody,
};
pub async fn create(Json(req): Json<UserBody<NewUser>>) -> Result<String> {
    let db = DB.get_or_init(db_conn).await;
    let _res = service::sys_user::create_user(db, req).await?;

    Ok("ok".to_string())
}

pub async fn login(Json(req): Json<UserBody<LoginUser>>) -> Result<Json<AuthBody>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::login(db, req).await?;
    Ok(Json(res))
}
