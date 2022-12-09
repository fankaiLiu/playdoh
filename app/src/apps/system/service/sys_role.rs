use ahash::HashMap;
use serde::Deserialize;
use sqlx::{PgPool, Transaction};



use sqlx::postgres::PgRow;
use sqlx::{types::Uuid, Pool, Postgres};
use anyhow::{anyhow,Result};

use super::sys_menu::MenuResp;
use super::sys_role_api;

async fn check_data_is_exist(db: &PgPool,role_name: String) -> Result<bool> {
    // let s1 = SysRole::find().filter(sys_role::Column::RoleName.eq(role_name));
    let count=sqlx::query_scalar!(
        r#"
        SELECT count(1) FROM "sys_role" WHERE role_name = $1"#,
        role_name,
    ).fetch_one(db).await?.unwrap_or_default();
    Ok(count > 0)
}

// pub async fn add(db: &PgPool, req: AddReq, user_id: &str) -> Result<String> {
//     if check_data_is_exist(&db,req.clone().role_name).await? {
//         return Err(anyhow!("数据已存在，请检查后重试"));
//     }

//     // 开启事务
//     let mut txn = db.begin().await?;
//     // 添加角色数据
//     //let role_id = self::add_role(txn, req.clone()).await?;
//     let role_id = add_role(&req, &mut txn).await?;
//     // 获取组合角色权限数据
//     let role_apis = self::get_permissions_data(&mut txn, role_id.clone(), req.menu_ids.clone()).await?;
//     // // 添加角色权限数据
//     // super::sys_role_api::add_role_api(&txn, role_apis, user_id).await?;

//     txn.commit().await?;
//     Ok("添加成功".to_string())
// }

async fn add_role(req: &AddReq, txn: &mut Transaction<'_,Postgres>) -> Result<Uuid, anyhow::Error> {
    let role_id = sqlx::query_scalar!(
        r#"
        INSERT INTO "sys_role" (role_name, role_key, list_order, data_scope, status, remark)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING role_id"#,
        req.role_name,
        req.role_key,
        req.list_order,
        req.data_scope.clone().unwrap_or_else(|| "3".to_string()),
        req.status,
        req.remark,
    ).fetch_one(txn).await?;
    Ok(role_id)
}

// // Combined Role Data
// pub async fn get_permissions_data(txn: &mut Transaction<'_,Postgres>, role_id: String, menu_ids: Vec<String>) -> Result<Vec<sys_role_api::AddReq>>
// {
//     // Get all menus are false
//     let menus = super::sys_menu::get_enabled_menus(txn, false, false).await?;
//     let menu_map = menus.iter().map(|x| (x.id.clone(), x.clone())).collect::<HashMap<String, MenuResp>>();
//     // Assemble role permission data
//     let mut res: Vec<sys_role_api::AddReq> = Vec::new();
//     for menu_id in menu_ids {
//         if let Some(menu) = menu_map.get(&menu_id) {
//             res.push(sys_role_api::AddReq {
//                 role_id: role_id.clone(),
//                 api: menu.api.clone(),
//                 method: Some(menu.method.clone()),
//             });
//         }
//     }
//     Ok(res)
// }

  

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