use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;
use anyhow::Result;

pub async fn get_dept_by_role_id(db: &PgPool, role_id: Uuid) -> Result<Vec<Uuid>> {
    let res=sqlx::query!(
            // language=PostgreSQL
            r#"select dept_id from sys_role_dept where role_id = $1"#,
            role_id
        ).fetch_all(db).await?.iter().map(|x|x.dept_id).collect();
    Ok(res)
}
