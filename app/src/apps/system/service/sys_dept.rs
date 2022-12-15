use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct DeptResp {
    pub dept_id: String,
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: NaiveDateTime,
    pub status: String,
}
