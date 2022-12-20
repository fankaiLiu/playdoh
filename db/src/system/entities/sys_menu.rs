use serde::{Deserialize, Serialize};
use sqlx::{FromRow, postgres::PgRow, Row};

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
