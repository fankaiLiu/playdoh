use anyhow::Result;
use db::{db_conn, DB, system::models::sys_user_online::SysUserOnline};
use sqlx::{types::Uuid, Pool, Postgres};
use time::OffsetDateTime;
use db::common::client::ClientInfo;


// // /// delete 完全删除
// // pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<String> {
// //     let mut s = SysUserOnline::delete_many();

// //     s = s.filter(sys_user_online::Column::Id.is_in(delete_req.ids));

// //     // 开始删除
// //     let d = s.exec(db).await?;

// //     match d.rows_affected {
// //         0 => Err(anyhow!("删除失败,数据不存在")),
// //         i => Ok(format!("成功删除{}条数据", i)),
// //     }
// // }

pub async fn check_online(db: &Pool<Postgres>, token_id: String) -> (bool, Option<SysUserOnline>) {
    let sys_user_online = sqlx::query_as!(
        SysUserOnline,
        r#"
        SELECT 
            id,
            user_id,
            token_id,
            token_exp,
            login_time,
            user_name,
            dept_name,
            net,
            ipaddr,
            login_location,
            device,
            browser,
            os
        FROM "sys_user_online" WHERE token_id = $1"#,
        token_id,
    )
    .fetch_optional(db)
    .await
    .unwrap();
    (sys_user_online.is_some(), sys_user_online)
}

// pub async fn log_out(db: &Pool<Postgres>, token_id: String) -> Result<String> {
//     sqlx::query!(
//         r#"
//         DELETE FROM "sys_oper_online" WHERE token_id = $1"#,
//         token_id,
//     )
//     .execute(db)
//     .await?;
//     Ok("成功退出登录".to_string())
// }


pub async fn add(req: ClientInfo, u_id: String, token_id: String, token_exp: i64) {
    let db = DB.get_or_init(db_conn).await;
    let u_id = Uuid::parse_str(&u_id).unwrap();
    let user = super::sys_user::get_by_id(db, &u_id)
        .await
        .expect("获取用户信息失败");
    //let dept = super::sys_dept::get_by_id(db, &user.clone().user.dept_id).await.expect("获取部门信息失败");
    let _id = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"INSERT INTO sys_user_online(
            user_id,token_id, token_exp, user_name, net, ipaddr, login_location, device, browser, os)  values (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id"#,
            u_id,
            token_id,
            token_exp,
            user.user.user_name,
            req.net.net_work,
            req.net.ip,
            req.net.location,
            req.ua.device,
            req.ua.browser,
            req.ua.os,
    )
    .fetch_one(db)
    .await;
}

// pub async fn update_online(token_id: String, token_exp: i64) -> Result<String> {
//     let db = DB.get_or_init(db_conn).await;
//     sqlx::query!(
//         r#"
//         UPDATE "sys_oper_online" SET token_exp = $1 WHERE token_id = $2"#,
//         token_exp,
//         token_id,
//     )
//     .execute(db)
//     .await?;
//     Ok("token更新成功".to_string())
// }

