use crate::{Result, ResponseResult, custom_response::CustomResponseBuilder};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, Pool, Postgres};

pub async fn create(db: &Pool<Postgres>, req: AddReq) -> ResponseResult<String> {
    let pid = Uuid::parse_str(&req.pid)?;
    let exist=check_router_is_exist_add(db, req.clone()).await?;
    if exist{
        let result =CustomResponseBuilder::new().status_code(StatusCode::BAD_REQUEST).body("路由已存在".to_string()).build();
        return Ok(result);
    }
    let _id = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"INSERT INTO public.sys_menu(
        pid, path, menu_name, icon, menu_type, query, order_sort, status, api, method, component, visible, is_cache, log_method, data_cache_method, is_frame, data_scope, i18n, remark)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13,$14,$15,$16,$17,$18,$19) returning id"#,
        pid,
        req.path,
        req.menu_name,
        req.icon,
        req.menu_type,
        req.query,
        req.order_sort,
        req.status,
        req.api,
        req.method,
        req.component,
        req.visible,
        req.is_cache,
        req.log_method,
        req.data_cache_method,
        req.is_frame,
        req.data_scope,
        req.i18n,
        req.remark,
    )
    .fetch_one(db)
    .await?;
    Ok(CustomResponseBuilder::new().body("添加成功".to_string()).build())
}

pub async fn update(db: &Pool<Postgres>, req: UpdateReq) -> ResponseResult<String> {
    let id = Uuid::parse_str(&req.id)?;
    let pid = Uuid::parse_str(&req.pid)?;
    sqlx::query!(
        // language=PostgreSQL
        r#"UPDATE public.sys_menu
        SET pid=$1, path=$2, menu_name=$3, icon=$4, menu_type=$5, query=$6, order_sort=$7, status=$8, api=$9, method=$10, component=$11, visible=$12, is_cache=$13, log_method=$14, data_cache_method=$15, is_frame=$16, data_scope=$17, i18n=$18, remark=$19
        WHERE id=$20"#,
        pid,
        req.path,
        req.menu_name,
        req.icon,
        req.menu_type,
        req.query,
        req.order_sort,
        req.status,
        req.api,
        req.method,
        req.component,
        req.visible,
        req.is_cache,
        req.log_method,
        req.data_cache_method,
        req.is_frame,
        req.data_scope,
        req.i18n,
        req.remark,
        id,
    )
    .execute(db)
    .await?;
    Ok(CustomResponseBuilder::new().body("ok".to_string()).build())
}

async fn check_router_is_exist_add(db: &Pool<Postgres>, req: AddReq) -> Result<bool> {
    let pid=Uuid::parse_str(&req.pid)?;
    let count1 = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"SELECT count(1) FROM public.sys_menu WHERE path=$1 AND pid=$2 AND menu_type<>'F'"#,
        req.path,
        pid
    ).fetch_one(db).await?.unwrap_or(0);
    let count2 = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"SELECT count(1) FROM public.sys_menu WHERE menu_name=$1 AND pid=$2"#,
        req.menu_name,
        pid
    ).fetch_one(db).await?.unwrap_or(0);
    Ok(count1 > 0 || count2 > 0)
}
#[derive(Deserialize, Clone)]
pub struct SearchReq {
    pub id: Option<String>,
    pub menu_name: Option<String>,
    pub menu_type: Option<String>,
    pub method: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}
#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub pid: String,
    pub path: Option<String>,
    pub menu_name: String,
    pub icon: Option<String>,
    pub menu_type: String,
    pub query: Option<String>,
    pub order_sort: i32,
    pub status: String,
    pub api: String,
    pub method: Option<String>,
    pub component: Option<String>,
    pub visible: String,
    pub is_frame: String,
    pub is_cache: String,
    pub data_scope: String,
    pub log_method: String,
    pub data_cache_method: String,
    pub i18n: Option<String>,
    pub remark: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateReq {
    pub id: String,
    pub pid: String,
    pub path: String,
    pub menu_name: String,
    pub icon: Option<String>,
    pub menu_type: String,
    pub query: Option<String>,
    pub order_sort: i32,
    pub status: String,
    pub api: String,
    pub method: Option<String>,
    pub component: String,
    pub visible: String,
    pub is_frame: String,
    pub is_cache: String,
    pub data_scope: String,
    pub log_method: String,
    pub data_cache_method: String,
    pub i18n: Option<String>,
    pub remark: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogCacheEditReq {
    pub id: String,
    pub log_method: String,
    pub data_cache_method: String,
}
#[derive(Serialize, Clone, Debug, Default)]
pub struct UserMenu {
    pub id: String,
    pub pid: String,
    pub always_show: Option<bool>,
    pub path: String,
    pub name: String,
    pub menu_name: String,
    pub menu_type: String,
    pub component: String,
    pub hidden: bool,
    pub meta: Meta,
}
#[derive(Serialize, Clone, Debug, Default)]
pub struct SysMenuTree {
    #[serde(flatten)]
    pub user_menu: UserMenu,
    pub children: Option<Vec<SysMenuTree>>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Meta {
    pub icon: String,
    pub title: String,
    pub link: Option<String>,
    pub no_cache: bool,
    pub hidden: bool,
    pub i18n: Option<String>,
}
pub struct SysMenu {
    pub id: Uuid,
    pub pid: Uuid,
    pub path: String,
    pub menu_name: String,
    pub icon: String,
    pub menu_type: String,
    pub query: Option<String>,
    pub order_sort: i32,
    pub status: String,
    pub api: String,
    pub method: String,
    pub component: String,
    pub visible: String,
    pub is_cache: String,
    pub log_method: String,
    pub data_cache_method: String,
    pub is_frame: String,
    pub data_scope: String,
    pub i18n: Option<String>,
    pub remark: String,
}
