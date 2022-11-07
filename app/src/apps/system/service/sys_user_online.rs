use sqlx::{Pool, Postgres};

pub async fn check_online(db: Option<&Pool<Postgres>>, id: String) -> (bool,Option<SysUserOnline>) {
    // let db = match db {
    //     Some(x) => x,
    //     None => DB.get_or_init(db_conn).await,
    // };

    // let model = SysUserOnline::find().filter(sys_user_online::Column::TokenId.eq(id)).one(db).await.expect("查询失败");

    (true,None )
}

pub struct SysUserOnline {
    pub id: String,
    pub u_id: String,
    pub token_id: String,
    pub token_exp: i64,
    //pub login_time: DateTime,
    pub user_name: String,
    pub dept_name: String,
    pub net: String,
    pub ipaddr: String,
    pub login_location: String,
    pub device: String,
    pub browser: String,
    pub os: String,
}