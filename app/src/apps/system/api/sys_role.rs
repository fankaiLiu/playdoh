use axum::{Json, extract::Query};
use db::{db_conn, DB, system::models::sys_role::*,system::models::sys_user::SearchReq as UserSearchReq};
use hyper::StatusCode;
use uuid::Uuid;

use crate::{apps::system::service::{self, }, utils::jwt::Claims, ResponseResult, custom_response::CustomResponseBuilder, pagination::PageParams};



pub async fn add(Json(req): Json<AddReq>, user: Claims) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::add(db, req, &user.id).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Ok(CustomResponseBuilder::new().body(e.to_string()).status_code(StatusCode::INTERNAL_SERVER_ERROR).build()),
    }
}


pub async fn delete(Json(delete_req): Json<DeleteReq>) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::delete(db, delete_req).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Ok(CustomResponseBuilder::new().body(e.to_string()).status_code(StatusCode::INTERNAL_SERVER_ERROR).build()),
    }
}

pub async fn edit(Json(edit_req): Json<EditReq>, user: Claims) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let uuid=Uuid::parse_str(&user.id)?;
    let res = service::sys_role::edit(db, edit_req, &uuid).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Ok(CustomResponseBuilder::new().body(e.to_string()).status_code(StatusCode::INTERNAL_SERVER_ERROR).build()),
    }
}

pub async fn change_status(Json(req): Json<StatusReq>) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::set_status(db, req).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Ok(CustomResponseBuilder::new().body(e.to_string()).status_code(StatusCode::INTERNAL_SERVER_ERROR).build()),
    }
}

pub async fn set_data_scope(Json(req): Json<DataScopeReq>) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::set_data_scope(db, req).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Ok(CustomResponseBuilder::new().body(e.to_string()).status_code(StatusCode::INTERNAL_SERVER_ERROR).build()),
    }
}



pub async fn get_by_id(Query(req): Query<SearchReq>) -> ResponseResult<Resp> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::get_by_id(db, req).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_all() -> ResponseResult<Vec<Resp>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::get_all(db).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Err(e.into()),
    }
}



pub async fn get_role_menu(Query(req): Query<SearchReq>) -> ResponseResult<Vec<Uuid>> {
    let db = DB.get_or_init(db_conn).await;
    match req.role_id {
        None => Ok(CustomResponseBuilder::new().body(vec![]).build()),
        Some(id) => {
            let api_ids = match service::sys_menu::get_role_permissions(db, &id).await {
                Ok((_, x)) => x,
                Err(e) => return Err(e.into()),
            };
            Ok(CustomResponseBuilder::new().body(api_ids).build())  
        }
    }
}



pub async fn get_role_dept(Query(req): Query<SearchReq>) -> ResponseResult<Vec<Uuid>> {
    match req.role_id {
        None => Ok(CustomResponseBuilder::new().body(vec![]).build()),
        Some(id) => {
            let db = DB.get_or_init(db_conn).await;
            let res = service::sys_dept::get_dept_by_role_id(db, id).await;
            match res {
                Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
                Err(e) => Err(e.into()),
            }
        }
    }
}
