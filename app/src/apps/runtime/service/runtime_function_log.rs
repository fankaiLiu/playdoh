use crate::{Result, pagination::{PageTurnResponse, PageParams, Pagination}};
use db::runtime::models::function_log::*;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
pub struct RuntimeFuctionLogService {}
pub type FnLogPageResponse = PageTurnResponse<FnLog>;

impl RuntimeFuctionLogService {
    pub fn new() -> Self {
        RuntimeFuctionLogService {}
    }
    pub async fn add_function_log(&self, db: &Pool<Postgres>, req: AddReq) -> Result<String> {
        {
            let id = sqlx::query_scalar!(
                // language=PostgreSQL
                r#"insert into "function_log" (function_name, start_time, end_time, status, execution_user_id, source, source_id, result_log, duration_ms, is_success, arguments) values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11) returning function_log_id"#,
                req.function_name,
                req.start_time,
                req.end_time,
                req.status,
                req.execution_user_id,
                req.source,
                req.source_id,
                req.result_log,
                req.duration_ms,
                req.is_success,
                req.arguments,
            )
            .fetch_one(db)
            .await?;
            // 	CallNumber+=1
            sqlx::query_scalar!(
                // language=PostgreSQL
                r#"update "sys_function_dev" set call_number=call_number+1 where function_dev_id=$1"#,
                req.source_id
            ).execute(db).await?;
            Ok(id.to_string())
        }
    }

    pub async fn page_function_log(&self,db: &Pool<Postgres>,id:&Uuid, page:PageParams) -> Result<FnLogPageResponse> {
        let pagination = Pagination::build_from_request_query(page).count(1).build();
        let res = sqlx::query_as!(
            FnLog,
            // language=PostgreSQL
            r#"SELECT function_log_id, function_name,TO_CHAR(start_time, 'YYYY-MM-DD HH24:MI:SS')as start_time, end_time, status, execution_user_id, source, source_id, result_log, duration_ms, is_success, arguments
            FROM public.function_log where source_id=$3 order by function_log_id desc limit $1 offset $2 "#,
            pagination.limit,
            pagination.offset,
            id
        )
        .fetch_all(db)
        .await?;
        let total = sqlx::query_scalar!(
            // language=PostgreSQL
            r#"select count(*) from "sys_function_dev""#,
        )
        .fetch_one(db)
        .await?;
        let page_turn = PageTurnResponse::new(total.unwrap_or_default(),pagination.limit, res );
        Ok(page_turn)
    }

}
