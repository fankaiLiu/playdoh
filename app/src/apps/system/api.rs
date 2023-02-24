use axum::{
    routing::{delete, get, post, put},
    Router,
};
mod sys_user; // 用户管理
mod sys_online;
mod sys_role;
mod sys_menu;
mod sys_home;
pub use sys_user::{login,login_check};

pub fn system_api() -> Router {
    Router::new().nest("/user", sys_user_api()) // 用户管理模块
    .nest("/role", sys_role_api())
    .nest("/menu", sys_menu_api())
    .nest("/", sys_home_api())
}

fn sys_user_api() -> Router {
    Router::new()
        .route("/", post(sys_user::create)) // 添加用户
        .route("/list", get(sys_user::list))
        .route("/", delete(sys_user::delete))
        .route("/", put(sys_user::update))
        .route("/logout", post(sys_online::log_out))
}
fn sys_role_api() -> Router {
    Router::new()
        //.route("/list", get(sys_role::get_sort_list)) // 获取筛选分页
        .route("/get_all", get(sys_role::get_all)) // 获取筛选分页
        .route("/get_by_id", get(sys_role::get_by_id)) // 按id获取
        .route("/add", post(sys_role::add)) // 添加
        .route("/edit", put(sys_role::edit)) // 更新
        .route("/update_auth_role", put(sys_role::update_auth_role)) // 更新角色授权
        .route("/change_status", put(sys_role::change_status)) // 设置状态
        .route("/set_data_scope", put(sys_role::set_data_scope)) // 设置数据权限范围
        .route("/delete", delete(sys_role::delete)) // 硬删除
        .route("/get_role_menu", get(sys_role::get_role_menu)) // 获取角色菜单
        .route("/get_role_dept", get(sys_role::get_role_dept)) // 获取角色部门
        .route("/cancel_auth_user", put(sys_role::cancel_auth_user)) // 批量用户取消角色授权
        .route("/add_auth_user", put(sys_role::add_auth_user)) // 批量用户角色授权
        .route("/get_auth_users_by_role_id", get(sys_role::get_auth_users_by_role_id)) // 获取角色对应用户
        .route("/get_un_auth_users_by_role_id", get(sys_role::get_un_auth_users_by_role_id))
}
fn sys_menu_api() -> Router {
    Router::new()
        //.route("/list", get(sys_menu::get_sort_list)) // 获取筛选分页
        // .route("/get_auth_list", get(sys_menu::get_auth_list)) // 权限查询列表
        .route("/get_by_id", get(sys_menu::get_by_id)) // 按id获取
        .route("/add", post(sys_menu::add)) // 添加
        .route("/edit", put(sys_menu::edit)) // 更新
        .route("/update_log_cache_method", put(sys_menu::update_log_cache_method)) // 更新api缓存方式和日志记录方式
        .route("/delete", delete(sys_menu::delete)) // 硬删除
        .route("/get_all_enabled_menu_tree", get(sys_menu::get_all_enabled_menu_tree)) // 获取全部正常的路由菜单树
        .route("/get_routers", get(sys_menu::get_routers)) // 获取用户菜单树
        .route("/get_auth_list", get(sys_menu::get_related_api_and_db)) // 获取用户菜单树
}

fn sys_home_api() -> Router {
    Router::new()
         .route("/", get(sys_home::home_page)) // 首页
         .route("/workspace", get(sys_home::workspace)) 
 }