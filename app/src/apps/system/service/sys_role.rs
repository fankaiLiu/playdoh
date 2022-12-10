use std::borrow::BorrowMut;
use std::str::FromStr;

use ahash::HashMap;
use db::db::{SqlCommandExecutor, TransactionManager};
use serde::Deserialize;
use sqlx::{Arguments, PgPool, Transaction};

use anyhow::{anyhow, Result};
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::{types::Uuid, Pool, Postgres};

use super::sys_menu::MenuResp;
use super::sys_role_api;

async fn check_data_is_exist(db: &PgPool, role_name: String) -> Result<bool> {
    // let s1 = SysRole::find().filter(sys_role::Column::RoleName.eq(role_name));
    let count = sqlx::query_scalar!(
        r#"
        SELECT count(1) FROM "sys_role" WHERE role_name = $1"#,
        role_name,
    )
    .fetch_one(db)
    .await?
    .unwrap_or_default();
    Ok(count > 0)
}

pub async fn add(db: &PgPool, req: AddReq, user_id: &str) -> Result<String> {
    if check_data_is_exist(&db, req.clone().role_name).await? {
        return Err(anyhow!("数据已存在，请检查后重试"));
    }

    // 开启事务
    let txn = db.begin().await?;
    let mut tranction_mannger = TransactionManager::new(txn);
    let mut exctor = SqlCommandExecutor::UseTransaction(tranction_mannger.borrow_mut());
    // 添加角色数据
    let role_id = add_role(&req, &mut exctor).await?;
    // 获取组合角色权限数据
    let role_apis =
        get_permissions_datar(&mut exctor, role_id.clone(), req.menu_ids.clone()).await?;
    // // 添加角色权限数据
    let uuid = Uuid::parse_str(user_id)?;
    super::sys_role_api::add_role_api(&mut exctor, role_apis, &uuid).await?;
    tranction_mannger.commit().await?;
    Ok("添加成功".to_string())
}
pub async fn delete(db: &PgPool, req: DeleteReq) -> Result<String> {
    let txn = db.begin().await?;
    let mut tranction_mannger = TransactionManager::new(txn);
    let mut exctor = SqlCommandExecutor::UseTransaction(tranction_mannger.borrow_mut());
    super::sys_role_api::delete_role_api(&mut exctor, req.role_ids.clone()).await?;
    let mut sql = String::from_str("DELETE  FROM  sys_role_api WHERE role_id in (")?;
    for i in 0..req.role_ids.len() {
        sql.push_str(&req.role_ids[i].to_string());
        if i != req.role_ids.len() - 1 {
            sql.push_str(",");
        }
    }
    sql.push_str(");");
    exctor.scalar_one(&sql).await?;
    tranction_mannger.commit().await?;
    Ok(format!("删除成功"))
}

async fn add_role<'a, 'b>(
    req: &AddReq,
    exector: &mut SqlCommandExecutor<'a, 'b>,
) -> Result<Uuid, anyhow::Error> {
    let mut args = PgArguments::default();
    args.add(&req.role_name);
    args.add(&req.role_key);
    args.add(req.list_order);
    args.add(&req.data_scope);
    args.add(&req.status);
    args.add(&req.remark);

    let role_id = exector
        .scalar_one_with(
            r#"
    INSERT INTO "sys_role" (role_name, role_key, list_order, data_scope, status, remark)
    VALUES ($1, $2, $3, $4, $5, $6)
    RETURNING role_id"#,
            args,
        )
        .await?;
    Ok(role_id)
}

// Combined Role Data
pub async fn get_permissions_datar<'a, 'b>(
    exector: &mut SqlCommandExecutor<'a, 'b>,
    role_id: Uuid,
    menu_ids: Vec<String>,
) -> Result<Vec<sys_role_api::AddReq>> {
    // Get all menus are false
    let menus = super::sys_menu::get_enabled_menus(exector, false, false).await?;
    let menu_map = menus
        .iter()
        .map(|x| (x.id.clone().unwrap_or_default(), x.clone()))
        .collect::<HashMap<String, MenuResp>>();
    //Assemble role permission data
    let mut res: Vec<sys_role_api::AddReq> = Vec::new();
    for menu_id in menu_ids {
        if let Some(menu) = menu_map.get(&menu_id) {
            res.push(sys_role_api::AddReq {
                role_id: role_id.to_string().clone(),
                api: menu.api.clone(),
                method: Some(menu.method.clone()),
                auth_type: menu.auth_type.clone(),
            });
        }
    }
    Ok(res)
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
