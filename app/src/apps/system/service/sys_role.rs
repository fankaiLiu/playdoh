use std::borrow::BorrowMut;
use std::str::FromStr;

use ahash::HashMap;
use db::db::{SqlCommandExecutor, TransactionManager};
use serde::Deserialize;
use sqlx::{Arguments, PgPool, Transaction};

use anyhow::{anyhow, Ok, Result};
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::{types::Uuid, Pool, Postgres};

use super::sys_menu::MenuResp;
use super::sys_role_api;

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
        get_permissions_data(&mut exctor, role_id.clone(), req.menu_ids.clone()).await?;
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
    let role_ids = req
        .role_ids
        .clone()
        .iter()
        .map(|x| Uuid::parse_str(x).unwrap())
        .collect();
    super::sys_role_api::delete_role_api(&mut exctor, role_ids).await?;
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

pub async fn edit(db: &PgPool, req: EditReq, created_by: &Uuid) -> Result<String> {
    //  检查字典类型是否存在
    let role_id = Uuid::parse_str(&req.role_id)?;
    if eidt_check_data_is_exist(db, role_id, req.clone().role_name, req.clone().role_key).await? {
        return Err(anyhow!("数据已存在"));
    }
    let txn = db.begin().await?;
    let mut tranction_mannger = TransactionManager::new(txn);
    let mut exctor = SqlCommandExecutor::UseTransaction(tranction_mannger.borrow_mut());

    let sql= String::from_str("UPDATE sys_role SET role_name=$1,role_key=$2,data_scope=$3,list_order=$4,status=$5,remark=$6,auth_type=$8 WHERE role_id=$9;")?;
    let mut args = PgArguments::default();
    args.add(&req.role_name);
    args.add(&req.role_key);
    args.add(&req.data_scope);
    args.add(&req.list_order);
    args.add(&req.status);
    args.add(&req.remark);
    args.add(&req.auth_type);
    args.add(&role_id);
    exctor.scalar_one_with(&sql, args).await?;

    let role_apis = get_permissions_data(&mut exctor, role_id, req.menu_ids.clone()).await?;
    super::sys_role_api::delete_role_api(&mut exctor, vec![role_id.clone()]).await?;

    // 添加角色权限数据
    super::sys_role_api::add_role_api(&mut exctor, role_apis, created_by).await?;

    tranction_mannger.commit().await?;
    Ok("角色数据更新成功".to_string())
}

pub async fn set_status(db:&PgPool,req: StatusReq)->Result<String>
{
    let role_id= Uuid::parse_str(&req.role_id)?;
    sqlx::query_scalar!("update sys_role set status=$1 where role_id=$2",req.status,role_id).execute(db).await?;
    Ok(format!("更新成功"))
}

// set_status 状态修改
pub async fn set_data_scope(db:&PgPool,req: DataScopeReq)->Result<String>
{

}

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

async fn eidt_check_data_is_exist(
    db: &PgPool,
    role_id: Uuid,
    role_name: String,
    role_key: String,
) -> Result<bool> {
    let count = sqlx::query_scalar!(
        r#"SELECT count(1) FROM "sys_role" WHERE role_name = $1 and role_id != $2"#,
        role_name,
        role_id
    )
    .fetch_one(db)
    .await?
    .unwrap_or_default();

    let count2 = sqlx::query_scalar!(
        r#"SELECT count(1) FROM "sys_role" WHERE role_name=$1 and role_id != $2 "#,
        role_name,
        role_id
    )
    .fetch_one(db)
    .await?
    .unwrap_or_default();
    Ok(count > 0 || count2 > 0)
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
pub async fn get_permissions_data<'a, 'b>(
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
