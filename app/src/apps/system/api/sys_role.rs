use axum::{Json, extract::Query};
use db::{db_conn, DB, system::models::{sys_role::*, sys_user::UserWithDept},system::models::sys_user::SearchReq as UserSearchReq};
use hyper::StatusCode;
use uuid::Uuid;

use crate::{apps::system::service::{self, sys_user::UserPageResponse, }, utils::jwt::Claims, ResponseResult, custom_response::CustomResponseBuilder, pagination::PageParams};



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

pub async fn get_auth_users_by_role_id(Query(mut req): Query<UserSearchReq>, Query(page_params): Query<PageParams>) -> ResponseResult<UserPageResponse> {
    let db = DB.get_or_init(db_conn).await;
    let role_id = match req.role_id.clone() {
        None => return Err(anyhow::anyhow!("role_id is required").into()),
        Some(id) => id,
    };
    let user_ids = match service::sys_role::get_auth_users_by_role_id(db, &role_id).await {
        Ok(x) => x,
        Err(e) => return Err(e.into()),
    };
    req.user_ids = Some(user_ids);
    let res = service::sys_user::page(page_params).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_un_auth_users_by_role_id(Query(mut req): Query<UserSearchReq>, Query(page_params): Query<PageParams>) -> ResponseResult<UserPageResponse> {
    let db = DB.get_or_init(db_conn).await;
    let role_id = match req.role_id.clone() {
        None => return  Err(anyhow::anyhow!("role_id is required").into()),
        Some(id) => id,
    };
    let user_ids = match service::sys_role::get_auth_users_by_role_id(db, &role_id).await {
        Ok(x) => x,
        Err(e) => return Err(e.into()),
    };
    req.user_ids = Some(user_ids);
    let res = service::sys_user::get_un_auth_user(db, page_params, req).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Err(e.into()),
    }
}

// edit 修改

pub async fn update_auth_role(Json(req): Json<UpdateAuthRoleReq>, user: Claims) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    match service::sys_role::add_role_by_user_id(db, &req.user_id, req.role_ids, user.id).await {
        Ok(_) =>  Ok(CustomResponseBuilder::new().body("更新成功".to_string()).build()),
        Err(e) => Err(e.into()),
    }
}

pub async fn add_auth_user(Json(req): Json<AddOrCancelAuthRoleReq>, user: Claims) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::add_role_with_user_ids(db, req.clone().user_ids, req.role_id, user.id).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body("添加成功".to_string()).build()),
        Err(e) => Err(e.into()),
    }
}

pub async fn cancel_auth_user(Json(req): Json<AddOrCancelAuthRoleReq>) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::cancel_auth_user(db, req).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body("ok".to_string()).build()),
        Err(e) => Err(e.into()),
    }
}
