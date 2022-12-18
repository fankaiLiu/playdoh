use db::{DB, db_conn};
use hyper::StatusCode;
use crate::{utils::jwt::Claims, ResponseResult, apps::system::service::sys_user_online, custom_response::CustomResponseBuilder};



pub async fn log_out(user: Claims) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = sys_user_online::log_out(db, user.token_id).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Ok(CustomResponseBuilder::new().body(e.to_string()).status_code(StatusCode::INTERNAL_SERVER_ERROR).build()),
    }
}