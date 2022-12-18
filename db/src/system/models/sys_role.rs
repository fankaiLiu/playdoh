use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub role_id: Option<Uuid>,
    pub role_ids: Option<Vec<String>>,
    pub role_key: Option<String>,
    pub role_name: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Resp {
    pub role_id: Option<String>,
    pub role_name: String,
    pub role_key: String,
    pub status: String,
    pub list_order: i32,
    pub remark: String,
    pub data_scope: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub role_name: String,
    pub role_key: String,
    pub list_order: i32,
    pub data_scope: Option<String>,
    pub status: String,
    pub remark: Option<String>,
    pub menu_ids: Vec<String>,
}
#[derive(Deserialize)]
pub struct DeleteReq {
    pub role_ids: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EditReq {
    pub role_id: String,
    pub role_name: String,
    pub role_key: String,
    pub list_order: i32,
    pub data_scope: String,
    pub auth_type: String,
    pub status: String,
    pub remark: Option<String>,
    pub menu_ids: Vec<String>,
}
#[derive(Deserialize, Clone)]
pub struct StatusReq {
    pub role_id: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct DataScopeReq {
    pub role_id: String,
    pub data_scope: String,
    pub dept_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, FromRow)]
pub struct SysRole {
    pub role_name: String,
    pub role_key: String,
    pub list_order: i32,
    pub data_scope: String,
    pub status: String,
    pub remark: String,
}

#[derive(Deserialize, Clone)]
pub struct AddOrCancelAuthRoleReq {
    pub user_ids: Vec<String>,
    pub role_id: String,
}
