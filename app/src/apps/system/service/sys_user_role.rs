use anyhow::Result;
use sqlx::{postgres::PgArguments, types::Uuid, Arguments, PgPool, Postgres, Transaction};

pub async fn get_user_ids_by_role_id(db: &PgPool, role_id: &Uuid) -> Result<Vec<Uuid>> {
    let ids = sqlx::query!(
        r#"
        SELECT user_id FROM sys_user_role WHERE role_id = $1
        "#,
        role_id
    )
    .fetch_all(db)
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|row| row.user_id)
            .collect()
    })?;
    Ok(ids)
}

// 删除用户角色
pub async fn delete_user_role<'a, 'b>(
    db: &'a mut Transaction<'_, Postgres>,
    user_id: &'b Uuid,
) -> Result<()> {
    // 先删除用户角色
    sqlx::query_scalar!(
        r#"
        DELETE FROM sys_user_role WHERE user_id = $1
        "#,
        user_id
    )
    .execute(db)
    .await?;
    Ok(())
}
// 添加修改用户角色
pub async fn edit_user_role<'a, 'b>(
    db: &'a mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    role_ids: Vec<Uuid>,
    created_by: &str,
) -> Result<()> {
    let create_by = Uuid::parse_str(created_by)?;
    let mut insert_sql =
        String::from("INSERT INTO sys_user_role (user_id, role_id, created_bys) VALUES ");
    let mut args = PgArguments::default();
    for (i, role_id) in role_ids.iter().enumerate() {
        insert_sql.push_str(format!("(${},${},{})", i * 3 + 1, i * 3 + 2, i * 3 + 3).as_str());
        if i != role_ids.len() - 1 {
            insert_sql.push_str(", ");
        }
        args.add(user_id);
        args.add(role_id);
        args.add(create_by);
    }
    insert_sql.push_str(r#" ON CONFLICT ON CONSTRAINT sys_role_user_pkey DO NOTHING"#);
    sqlx::query_scalar_with(insert_sql.as_str(), args)
        .fetch_one(db)
        .await?;
    Ok(())
}

pub async fn add_role_by_lot_user_ids(
    db: &mut Transaction<'_, Postgres>,
    user_ids: Vec<String>,
    role_id: String,
    created_by: &str,
) -> Result<()> {
    let role_id = Uuid::parse_str(role_id.as_str())?;
    let create_by = Uuid::parse_str(created_by)?;
    let mut insert_sql =
        String::from("INSERT INTO sys_user_role (user_id, role_id, created_bys) VALUES ");
    let mut args = PgArguments::default();
    for (i, user_id) in user_ids.iter().enumerate() {
        insert_sql.push_str(format!("(${},${},{})", i * 3 + 1, i * 3 + 2, i * 3 + 3).as_str());
        if i != user_ids.len() - 1 {
            insert_sql.push_str(", ");
        }
        args.add(user_id);
        args.add(role_id);
        args.add(create_by);
    }
    insert_sql.push_str(r#" ON CONFLICT ON CONSTRAINT sys_role_user_pkey DO NOTHING"#);
    sqlx::query_scalar_with(insert_sql.as_str(), args)
        .fetch_one(db)
        .await?;
    Ok(())
}

// 批量删除某个角色的多个用户
pub async fn delete_user_role_by_user_ids(
    db: &mut Transaction<'_, Postgres>,
    user_ids: Vec<String>,
    role_id: Option<String>,
) -> Result<()> {
    let mut delete_sql = String::from("DELETE FROM sys_user_role WHERE user_id IN (");
    let mut args = PgArguments::default();
    for (i, user_id) in user_ids.iter().enumerate() {
        delete_sql.push_str(format!("${}", i + 1).as_str());
        if i != user_ids.len() - 1 {
            delete_sql.push_str(", ");
        }
        args.add(user_id);
    }
    if let Some(role_id) = role_id {
        delete_sql.push_str(") AND role_id = $");
        delete_sql.push_str((user_ids.len() + 1).to_string().as_str());
        args.add(role_id);
    } else {
        delete_sql.push_str(")");
    }
    sqlx::query_scalar_with(delete_sql.as_str(), args)
    .fetch_one(db)
    .await?;
    Ok(())
}
