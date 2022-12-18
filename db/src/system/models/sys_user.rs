use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::sys_dept::DeptResp;


#[derive(serde::Deserialize)]
pub struct NewUser {
    pub user_name: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateUser {
    id: String,
    username: String,
    email: String,
    password: String,
    bio: String,
    image: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub token: String,
    pub user_name: String,
}


#[derive(Debug, Deserialize, Clone,Serialize)]
pub struct UserResp {
    pub user_id: Uuid,
    pub email: String,
    pub user_name: String,
    pub bio: String,
    pub user_nickname: Option<String>,
    pub gender: i64,
    pub dept_id: Option<Uuid>,
    pub remark: Option<String>,
    pub is_admin: i32,
    pub phone_num: Option<String>,
    pub role_id: Option<Uuid>,
    pub created_at:time::OffsetDateTime,
}


#[derive(serde::Deserialize)]
pub struct SearchResult {
    pub user_name: String,
    pub email: String,
}


#[derive(serde::Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
#[derive(Debug, Clone,Deserialize,Serialize)]
pub struct UserWithDept {
    #[serde(flatten)]
    pub user: UserResp,
    pub dept: DeptResp,
}
impl UserWithDept {
    pub fn new(user: UserResp, dept: DeptResp) -> Self {
        Self { user, dept }
    }
}