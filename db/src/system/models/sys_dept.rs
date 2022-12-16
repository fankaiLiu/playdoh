use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeptResp {
    pub dept_id: Uuid,
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: time::OffsetDateTime,
    pub status: String,
}