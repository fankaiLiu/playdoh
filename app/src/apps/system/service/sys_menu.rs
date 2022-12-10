use crate::pagination::{PageParams, PageTurnResponse, Pagination};
use crate::{custom_response::CustomResponseBuilder, utils};
use anyhow::anyhow;
use anyhow::Result;
use db::db::SqlCommandExecutor;
use db::{db_conn, DB};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{types::Uuid, Pool, Postgres};
use sqlx::{FromRow, QueryBuilder, Row};

pub struct SysMenuService<'a, 'b> {
    sql_command_executor: &'b mut SqlCommandExecutor<'a, 'b>,
}

impl<'a, 'b> SysMenuService<'a, 'b> {
    pub fn new(sql_command_executor: &'b mut SqlCommandExecutor<'a, 'b>) -> Self {
        Self {
            sql_command_executor,
        }
    }
}

pub async fn create(db: &Pool<Postgres>, req: AddReq) -> Result<String> {
    let pid = Uuid::parse_str(&req.pid)?;
    let exist = check_router_is_exist_add(db, req.clone()).await?;
    if exist {
        return Err(anyhow!("路由已存在"));
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
    Ok("添加成功".to_string())
}

pub async fn update(db: &Pool<Postgres>, req: UpdateReq) -> Result<String> {
    if check_router_is_exist_update(db, req.clone()).await? {
        return Err(anyhow!("路径或者名称重复"));
    }
    let id = Uuid::parse_str(&req.id)?;
    let pid = Uuid::parse_str(&req.pid)?;
    //Check if the route exists
    let exist = sqlx::query!(
        // language=PostgreSQL
        r#"select  id from public.sys_menu where id = $1 limit 1"#,
        id
    )
    .fetch_optional(db)
    .await?;
    if exist.is_none() {
        return Err(anyhow!("路由不存在"));
    }
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
    Ok("修改成功".to_string())
}

pub async fn delete(db: &Pool<Postgres>, id: &str) -> Result<String> {
    let id = Uuid::parse_str(id)?;
    let count = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"select api from public.sys_menu where pid = $1"#,
        id
    )
    .fetch_optional(db)
    .await?;
    match count {
        Some(api) => {
            let mut txn = db.begin().await?;
            sqlx::query!(
                // language=PostgreSQL
                r#"delete from public.sys_menu where id = $1"#,
                id
            )
            .execute(&mut txn)
            .await?;
            utils::ApiUtils::remove_api(&api).await;
            txn.commit().await?;
            Ok("删除成功".to_string())
        }
        None => {
            let result = CustomResponseBuilder::new()
                .status_code(StatusCode::BAD_REQUEST)
                .body("路由不存在".to_string())
                .build();
            return Err(anyhow!("请求参数错误"));
        }
    }
}
/// 更新日志和缓存方法
pub async fn update_log_cache_method(db: &Pool<Postgres>, req: LogCacheEditReq) -> Result<String> {
    let id = Uuid::parse_str(&req.id)?;
    sqlx::query!(
        // language=PostgreSQL
        r#"UPDATE public.sys_menu
        SET log_method=$1, data_cache_method=$2
        WHERE id=$3"#,
        req.log_method,
        req.data_cache_method,
        id,
    )
    .fetch_one(db)
    .await?;
    Ok("更新成功".to_string())
}

pub async fn get_by_id(db: &Pool<Postgres>, search_req: SearchReq) -> Result<MenuResp> {
    if let Some(id) = search_req.id {
        let id = Uuid::parse_str(&id)?;
        let result = sqlx::query_as!(
            MenuResp,
            // language=PostgreSQL uuid type to string type
            r#"select id::text, pid::text, path, menu_name, icon, menu_type, query, order_sort, status, api, method, component, visible, is_cache, log_method, data_cache_method, is_frame, data_scope, i18n, remark ,auth_type from public.sys_menu where deleted_at is null and id = $1"#,
            id
        ).fetch_one(db).await?;
        return Ok(result);
    } else {
        return Err(anyhow!("请求参数错误"));
    }
}

/// get_all 获取全部
/// db 数据库连接 使用db.0
pub async fn get_all_router_tree(db: &Pool<Postgres>) -> Result<Vec<SysMenuTree>> {
    let mut db=SqlCommandExecutor::WithoutTransaction(db);
    let menus = get_enabled_menus(&mut db, true, false).await?;
    let menu_data = self::get_menu_data(menus);
    let menu_tree = self::get_menu_tree(menu_data, "0".to_string());

    Ok(menu_tree)
}

pub fn get_menu_tree(user_menus: Vec<SysMenuTree>, pid: String) -> Vec<SysMenuTree> {
    let mut menu_tree: Vec<SysMenuTree> = Vec::new();
    for mut user_menu in user_menus.clone() {
        if user_menu.user_menu.pid == pid {
            user_menu.children = Some(get_menu_tree(
                user_menus.clone(),
                user_menu.user_menu.id.clone(),
            ));
            menu_tree.push(user_menu.clone());
        }
    }
    menu_tree
}
pub fn get_menu_data(menus: Vec<MenuResp>) -> Vec<SysMenuTree> {
    let mut menu_res: Vec<SysMenuTree> = Vec::new();
    for mut menu in menus {
        menu.pid = Some(menu.pid.unwrap_or_default().trim().to_string());
        let meta = Meta {
            icon: menu.icon.clone(),
            title: menu.menu_name.clone(),
            hidden: menu.visible.clone() != "1",
            link: if menu.path.clone().starts_with("http") {
                Some(menu.path.clone())
            } else {
                None
            },
            no_cache: menu.is_cache.clone() != "1",
            i18n: menu.i18n,
        };
        let user_menu = UserMenu {
            meta,
            id: menu.id.unwrap_or_default().clone(),
            pid: menu.pid.clone().unwrap_or_default(),
            path: if !menu.path.clone().starts_with('/')
                && menu.pid.clone().unwrap_or_default() == "0"
            {
                format!("/{}", menu.path.clone())
            } else {
                menu.path.clone()
            },
            name: menu.path.clone(),
            menu_name: menu.menu_name.clone(),
            menu_type: menu.menu_type.clone(),
            always_show: if menu.is_cache.clone() == "1"
                && menu.pid.clone().unwrap_or_default() == "0"
            {
                Some(true)
            } else {
                None
            },
            component: menu.component.clone(),
            hidden: menu.visible.clone() == "0",
        };
        let menu_tree = SysMenuTree {
            user_menu,
            ..Default::default()
        };
        menu_res.push(menu_tree);
    }
    menu_res
}

pub async fn get_related_api_by_db_name(db: &Pool<Postgres>, api_id: &str) -> Result<Vec<String>> {
    let api_id = Uuid::parse_str(api_id)?;
    let apis= sqlx::query!(
        // language=PostgreSQL
        r#"select sys_menu.api from public.sys_menu where deleted_at is null and method = 'GET' and 
        id in (select api_id from public.sys_api_db where db in (select db from public.sys_api_db where api_id = $1))"#,
        api_id
    ).fetch_all(db).await?;
    let mut res = Vec::new();
    for api in apis {
        res.push(api.api);
    }
    Ok(res)
}

pub async fn get_related_db_by_db_name(db: &Pool<Postgres>, api_id: &str) -> Result<Vec<String>> {
    let api_id = Uuid::parse_str(api_id)?;
    //sys_api_db::Entity::find().filter(sys_api_db::Column::ApiId.eq(item.id.clone())).all(db)
    let apis = sqlx::query!(
        // language=PostgreSQL
        r#"select sys_api_db.db from public.sys_api_db where  api_id =$1"#,
        api_id
    )
    .fetch_all(db)
    .await?;
    let mut res = Vec::new();
    for api in apis {
        res.push(api.db);
    }
    Ok(res)
}
pub struct MenuClient {}
impl MenuClient {
    pub fn new() -> Self {
        Self {}
    }
}
pub type MenuPageResponse = PageTurnResponse<MenuResp>;
async fn page(page_params: PageParams, search_req: Option<SearchReq>) -> Result<MenuPageResponse> {
    let db = DB.get_or_init(db_conn).await;
    let pagination = Pagination::build_from_request_query(page_params).build();
    let mut count_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "select count(1) as count from public.sys_menu where deleted_at is null ",
    );
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "select cast(id as varchar) ,  cast(pid as varchar), path, auth_type,menu_name, icon, menu_type, query, order_sort, status, api, method, component, visible, is_cache, log_method, data_cache_method, is_frame, data_scope, i18n, remark from public.sys_menu where deleted_at is null "
        );
    if let Some(filter) = &search_req {
        if let Some(name) = &filter.menu_name {
            query_builder
                .push(" and menu_name like concat('%', ")
                .push_bind(name.clone());
            query_builder.push(", '%')");
            count_builder
                .push(" and menu_name like concat('%', ")
                .push_bind(name.clone());
            count_builder.push(", '%')");
        }
        if let Some(method) = &filter.method {
            query_builder
                .push(" and method = '")
                .push_bind(method.clone());
            query_builder.push("'");
            count_builder
                .push(" and method = '")
                .push_bind(method.clone());
            count_builder.push("'");
        }
        if let Some(component) = &filter.status {
            query_builder
                .push(" and component = '")
                .push_bind(component.clone());
            query_builder.push("'");
            count_builder
                .push(" and component = '")
                .push_bind(component.clone());
            count_builder.push("'");
        }
        if let Some(api) = &filter.menu_type {
            query_builder.push(" and api = '").push_bind(api.clone());
            query_builder.push("'");
            count_builder.push(" and api = '").push_bind(api.clone());
            count_builder.push("'");
        }
        if let Some(begin_time) = &filter.begin_time {
            query_builder
                .push(" and begin_time <= '")
                .push_bind(begin_time.clone());
            query_builder.push("'");
            count_builder
                .push(" and begin_time <= '")
                .push_bind(begin_time);
            count_builder.push("'");
        }
        if let Some(end_time) = &filter.end_time {
            query_builder
                .push(" and end_time >= '")
                .push_bind(end_time.clone());
            query_builder.push("'");
            count_builder
                .push(" and end_time >= '")
                .push_bind(end_time.clone());
            count_builder.push("'");
        }
    }
    let result = query_builder.build().fetch_all(db).await?;
    let menus = result
        .iter()
        .map(|x| MenuResp::from_row(x))
        .collect::<Result<Vec<MenuResp>, _>>()?;
    let count: i64 = count_builder
        .build()
        .fetch_one(db)
        .await?
        .try_get("count")?;
    return Ok(MenuPageResponse::new(count, menus));
}

pub type MenuRelatedPageResponse = PageTurnResponse<MenuRelated>;

pub async fn get_related_api_and_db(
    db: &Pool<Postgres>,
    pag_params: PageParams,
    search_req: Option<SearchReq>,
) -> Result<MenuRelatedPageResponse> {
    let menus = self::page(pag_params, search_req).await?;
    let mut res: Vec<MenuRelated> = Vec::new();
    for item in menus.data.into_iter() {
        let id = &item.clone().id.unwrap_or_default();
        let (dbs_model, apis) = tokio::join!(
            self::get_related_db_by_db_name(db, id),
            self::get_related_api_by_db_name(db, id),
        );
        let dbs = dbs_model?;
        let apis = match apis {
            Ok(v) => v,
            Err(e) => return Err(anyhow!("{}", e)),
        };
        res.push(MenuRelated {
            menu: item,
            dbs,
            apis,
        });
    }
    return Ok(MenuRelatedPageResponse::new(menus.total_count, res));
}

/// 获取全部菜单 或者 除开按键api级别的外的路由
/// is_router 是否是菜单路由，用于前端生成路由
/// is_only_api 仅获取按键，api级别的路由
/// 不能同时为true
/// 同时false 为获取全部路由
pub async fn get_enabled_menus<'a, 'b>(
    exector: & mut SqlCommandExecutor<'a, 'b>,
    is_router: bool,
    is_only_api: bool,
) -> Result<Vec<MenuResp>> {
    if is_only_api && is_router {
        return Err(anyhow!("请求参数错误"));
    }
    let mut sql = String::new();
    if is_router {
        sql = String::from(
            r#"select id::text, pid::text, path, menu_name, icon, menu_type, query, order_sort, status, api, method, component, visible, is_cache, log_method, data_cache_method, is_frame, data_scope, i18n, remark from public.sys_menu where deleted_at is null and status = '1' and menu_type <> 'F' order by order_sort "#,
        );
    } else if is_only_api {
        sql = String::from(
            r#"select id::text, pid::text, path, menu_name, icon, menu_type, query, order_sort, status, api, method, component, visible, is_cache, log_method, data_cache_method, is_frame, data_scope, i18n, remark from public.sys_menu where deleted_at is null and status = '1' and menu_type = 'F' order by order_sort "#,
        );
    } else {
        sql = String::from(
            r#"select id::text, pid::text, path, menu_name, icon, menu_type, query, order_sort, status, api, method, component, visible, is_cache, log_method, data_cache_method, is_frame, data_scope, i18n, remark from public.sys_menu where deleted_at is null and status = '1' order by order_sort "#,
        );
    }
    Ok(exector.find_all(&sql).await?)
}

async fn check_router_is_exist_add(db: &Pool<Postgres>, req: AddReq) -> Result<bool> {
    let pid = Uuid::parse_str(&req.pid)?;
    let count1 = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"SELECT count(1) FROM public.sys_menu WHERE path=$1 AND pid=$2 AND menu_type<>'F'"#,
        req.path,
        pid
    )
    .fetch_one(db)
    .await?
    .unwrap_or(0);
    let count2 = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"SELECT count(1) FROM public.sys_menu WHERE menu_name=$1 AND pid=$2"#,
        req.menu_name,
        pid
    )
    .fetch_one(db)
    .await?
    .unwrap_or(0);
    Ok(count1 > 0 || count2 > 0)
}
async fn check_router_is_exist_update(db: &Pool<Postgres>, req: UpdateReq) -> Result<bool> {
    let pid = Uuid::parse_str(&req.pid)?;
    let id = Uuid::parse_str(&req.id)?;

    let count1= sqlx::query_scalar!(
        // language=PostgreSQL
        r#"SELECT count(1) FROM public.sys_menu WHERE path=$1 AND pid=$2 AND menu_type<>'F' AND id<>$3"#,
        req.path,
        pid,
        id
    ).fetch_one(db).await?.unwrap_or(0);

    let count2 = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"SELECT count(1) FROM public.sys_menu WHERE menu_name=$1 AND pid=$2 AND id<>$3"#,
        req.menu_name,
        pid,
        id
    )
    .fetch_one(db)
    .await?
    .unwrap_or(0);
    Ok(count1 > 0 || count2 > 0)
}
#[derive(Serialize, Clone, Debug)]
pub struct MenuRelated {
    #[serde(flatten)]
    pub menu: MenuResp,
    pub dbs: Vec<String>,
    pub apis: Vec<String>,
}

#[derive(Deserialize, Clone, Default)]
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
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MenuResp {
    pub id: Option<String>,
    pub pid: Option<String>,
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
    pub is_frame: String,
    pub is_cache: String,
    pub data_scope: String,
    pub log_method: String,
    pub auth_type: String,
    pub i18n: Option<String>,
    pub data_cache_method: String,
    pub remark: String,
}
impl<'r> FromRow<'r, PgRow> for MenuResp {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(MenuResp {
            id: row.try_get("id")?,
            pid: row.try_get("pid")?,
            path: row.try_get("path")?,
            menu_name: row.try_get("menu_name")?,
            icon: row.try_get("icon")?,
            menu_type: row.try_get("menu_type")?,
            query: row.try_get("query")?,
            order_sort: row.try_get("order_sort")?,
            status: row.try_get("status")?,
            api: row.try_get("api")?,
            method: row.try_get("method")?,
            component: row.try_get("component")?,
            visible: row.try_get("visible")?,
            is_frame: row.try_get("is_frame")?,
            is_cache: row.try_get("is_cache")?,
            data_scope: row.try_get("data_scope")?,
            log_method: row.try_get("log_method")?,
            i18n: row.try_get("i18n")?,
            data_cache_method: row.try_get("data_cache_method")?,
            remark: row.try_get("remark")?,
            auth_type: row.try_get("auth_type")?,
        })
    }
}
