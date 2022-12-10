use anyhow::Result;
use db::db::SqlCommandExecutor;
use serde::Deserialize;
use sqlx::{postgres::PgArguments, types::Uuid, Arguments};

// 添加修改用户角色
pub async fn add_role_api<'a, 'b>(
    exector: &mut SqlCommandExecutor<'a, 'b>,
    role_apis: Vec<AddReq>,
    created_by: &Uuid,
) -> Result<()> {
    if !role_apis.is_empty() {
        let mut args = PgArguments::default();
        let mut sql = r#"
        INSERT INTO "sys_role_api" ( role_id, api, method, created_by, auth_type)
        VALUES "#
            .to_string();
        for (i, x) in role_apis.iter().enumerate() {
            sql.push_str(&format!(
                "({}, {}, {}, {}, {})",
                i * 6 + 1,
                i * 6 + 2,
                i * 6 + 3,
                i * 6 + 4,
                i * 6 + 5,
            ));
            if i != role_apis.len() - 1 {
                sql.push_str(",");
            }
            args.add(&x.role_id);
            args.add(&x.api);
            args.add(&x.method);
            args.add(&created_by);
            args.add(&x.auth_type);
        }
        exector.execute_with(&sql, args).await?;
    }
    Ok(())
}

pub async fn delete_role_api<'a, 'b>(
    exector: &mut SqlCommandExecutor<'a, 'b>,
    role_ids: Vec<String>,
) -> Result<()> {
    //sys_role_api::Entity::delete_many().filter(sys_role_api::Column::RoleId.is_in(role_ids)).exec(db).await?;
    let mut sql = String::from("DELETE FROM sys_role_api WHERE role_id IN (");
    for (i, id) in role_ids.iter().enumerate() {
        sql.push_str(&format!("{}, ", id));
        if i != role_ids.len() - 1 {
            sql.push_str(",");
        }
    }
    sql.push_str(")");
    exector.execute(&sql).await?;
    Ok(())
}

// api 格式 （api，method）
pub async fn update_api<'a, 'b>(
    exector: &mut SqlCommandExecutor<'a, 'b>,
    old_api: (&str, &str),
    new_api: (&str, &str),
) -> Result<()> {
    let  sql= String::from("UPDATE sys_role_api SET api = $1, method = $2 WHERE api = $3 AND method = $4 where api =$5 and method = $6");
    let mut args = PgArguments::default();
    args.add(&new_api.0);
    args.add(&new_api.1);
    args.add(&old_api.0);
    args.add(&old_api.1);
    exector.execute_with(&sql, args).await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct AddReq {
    pub role_id: String,
    pub api: String,
    pub method: Option<String>,
    pub auth_type: String,
}
