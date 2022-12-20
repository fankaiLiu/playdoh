use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::system::entities::sys_menu::MenuResp;

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
    pub pid: Option<Uuid>,
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
 
#[derive(Debug, Deserialize)]
pub struct DeleteReq {
    pub id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateReq {
    pub id: Uuid,
    pub pid: Option<Uuid>,
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
 