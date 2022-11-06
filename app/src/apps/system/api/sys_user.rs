use axum::Json;
use db::{db_conn, DB};

use crate::{
    apps::system::service::{
        self,
        sys_user::{NewUser, UserBody, LoginUser, User},
    },
    Result,
};
pub async fn create(Json(req): Json<UserBody<NewUser>>) -> Result<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::create_user(db, req).await?;

    Ok("ok".to_string())
}

pub async fn login_user(
    Json(req): Json<UserBody<LoginUser>>,
) -> Result<Json<UserBody<User>>> {
    let db = DB.get_or_init(db_conn).await;
    let res=service::sys_user::login_user(db, req).await?;
    Ok(res)
}