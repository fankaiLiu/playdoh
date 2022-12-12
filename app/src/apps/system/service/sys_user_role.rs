use anyhow::Result;
use sqlx::{types::Uuid, PgPool, Transaction, Postgres};

pub async fn get_user_ids_by_role_id(db: &PgPool, role_id: &str) -> Result<Vec<String>> {
    let role_id = Uuid::parse_str(role_id)?;
    let ids = sqlx::query!(
        r#"
        SELECT user_id FROM sys_role_user WHERE role_id = $1
        "#,
        role_id
    )
    .fetch_all(db)
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|row| row.user_id.to_string())
            .collect()
    })?;
    Ok(ids)
}

// 删除用户角色
pub async fn delete_user_role<'a,'b>(db: &'a mut  Transaction<'_,Postgres>, user_id: &'b  str) -> Result<()>
{
    // 先删除用户角色
    let user_id= Uuid::parse_str(user_id)?;
    sqlx::query_scalar!(
        r#"
        DELETE FROM sys_role_user WHERE user_id = $1
        "#,
        user_id
    ).execute(db).await?;
    Ok(())
}
// 添加修改用户角色
// pub async fn edit_user_role<'a,'b>(db: &'a mut  Transaction<'_,Postgres>, user_id: &str, role_ids: Vec<String>, created_by: &str) -> Result<()>{
//     // 添加用户角色
//     // sys_user_role::Entity::insert_many(
//     //     role_ids
//     //         .clone()
//     //         .iter()
//     //         .map(|x| sys_user_role::ActiveModel {
//     //             id: Set(scru128::new_string()),
//     //             user_id: Set(user_id.to_string()),
//     //             role_id: Set(x.to_string()),
//     //             created_by: Set(created_by.to_string()),
//     //             created_at: Set(Local::now().naive_local()),
//     //         })
//     //         .collect::<Vec<_>>(),
//     // )
//     // .exec(db)
//     // .await?;
//     let user_id=Uuid::parse_str(user_id)?;
//     let create_by=Uuid::parse_str(created_by)?;
//     let mut insert_sql = String::from("INSERT INTO sys_role_user (user_id, role_id, created_bys) VALUES ");
//     for (i,role_id) in role_ids.iter().enumerate() {
//         insert_sql.push_str(r#"($1, $2, $3)"#);
//         if i!= role_ids.len() - 1 {
//             insert_sql.push_str(", ");
//         }        
//     }
//     insert_sql.push_str(r#" ON CONFLICT ON CONSTRAINT sys_role_user_pkey DO NOTHING"#);



// }