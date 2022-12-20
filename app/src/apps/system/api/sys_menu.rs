use axum::{extract::Query, Json};
use configs::CFG;
use db::{
    db_conn,
    system::{
        entities::sys_menu::*,
        models::sys_menu::*,
    },
    DB,
};
use hyper::StatusCode;
use uuid::Uuid;

use super::super::service;
use crate::{utils::jwt::Claims, pagination::{PageParams, PageTurnResponse}, custom_response::{CustomResponseBuilder}, ResponseResult};

/// get_all_menu_tree 获取全部菜单

// pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(search_req): Query<SearchReq>) -> ResponseResult<PageTurnResponse<MenuResp>> {
//     let db = DB.get_or_init(db_conn).await;
//     let res = service::sys_menu::get_sort_list(db, page_params, search_req).await;
//     match res {
//         Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
//         Err(e) => Ok(CustomResponseBuilder::new().body(e.to_string()).status_code(StatusCode::INTERNAL_SERVER_ERROR).build()),
//     }
// }

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_id(Query(search_req): Query<SearchReq>) -> ResponseResult<MenuResp> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_by_id(db, search_req).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) =>Err(e.into()),
    }
}

/// add 添加

pub async fn add(Json(req): Json<AddReq>) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::create(db, req).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Ok(CustomResponseBuilder::new().body(e.to_string()).status_code(StatusCode::INTERNAL_SERVER_ERROR).build()),
    }
}

/// delete 完全删除

pub async fn delete(Json(req): Json<DeleteReq>) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::delete(db, &req.id).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Ok(CustomResponseBuilder::new().body(e.to_string()).status_code(StatusCode::INTERNAL_SERVER_ERROR).build()),
    }
}

// edit 修改

pub async fn edit(Json(update_req): Json<UpdateReq>) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::update(db, update_req).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Ok(CustomResponseBuilder::new().body(e.to_string()).status_code(StatusCode::INTERNAL_SERVER_ERROR).build()),
    }
}
// update_log_cache_method 修改菜单日志缓存方法
pub async fn update_log_cache_method(Json(edit_req): Json<LogCacheEditReq>) -> ResponseResult<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::update_log_cache_method(db, edit_req).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) => Ok(CustomResponseBuilder::new().body(e.to_string()).status_code(StatusCode::INTERNAL_SERVER_ERROR).build()),
    }
}

/// get_all_menu_tree 获取全部菜单树

pub async fn get_all_enabled_menu_tree() -> ResponseResult<Vec<SysMenuTree>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_all_enabled_menu_tree(db).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) =>Err(e.into()),
    }
}

/// get_related_api_and_db 获取全部菜单树
pub async fn get_related_api_and_db(Query(page_params): Query<PageParams>, Query(search_req): Query<SearchReq>) -> ResponseResult<PageTurnResponse<MenuRelated>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_related_api_and_db(db, page_params, Some(search_req)).await;
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) =>Err(e.into()),
    }
}

/// 获取用户路由
pub async fn get_routers(user: Claims) -> ResponseResult<Vec<SysMenuTree>> {
    let db = DB.get_or_init(db_conn).await;
    //  获取 用户角色
    let user_id=Uuid::parse_str(&user.id)?;
    let role_id = service::sys_role::get_current_admin_role(db, &user_id).await;
    if role_id.is_none() {
        return  Err(anyhow::anyhow!("用户角色不存在").into());
    }
    let role_id = role_id.unwrap();

    // 检查是否超管用户
    let res = if CFG.system.super_user.contains(&user.id) {
        service::sys_menu::get_all_router_tree(db).await
    } else {
        service::sys_menu::get_admin_menu_by_role_ids(db, &role_id).await
    };
    match res {
        Ok(x) =>  Ok(CustomResponseBuilder::new().body(x).build()),
        Err(e) =>Err(e.into()),
    }
}
