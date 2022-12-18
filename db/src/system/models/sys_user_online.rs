use time::OffsetDateTime;
use uuid::Uuid;

pub struct SysUserOnline {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_id: String,
    pub token_exp: i64,
    pub login_time: OffsetDateTime,
    pub user_name: String,
    pub dept_name: Option<String>,
    pub net: String,
    pub ipaddr: String,
    pub login_location: String,
    pub device: String,
    pub browser: String,
    pub os: String,
}
