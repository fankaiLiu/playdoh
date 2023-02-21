use crate::{
    pagination::{PageParams, PageTurnResponse, Pagination},
    Result,
};
use db::runtime::entities::{function::Function, function_history::FunctionHistory};
use sqlx::PgPool;
use uuid::Uuid;

pub struct RuntimeFuctionHistoryService {}
pub type FnHistoryPageResponse = PageTurnResponse<FunctionHistory>;

impl RuntimeFuctionHistoryService {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn add_function_history(
        &self,
        db: &PgPool,
        function: Function,
        tag: Option<String>,
    ) -> Result<String> {
        let id = sqlx::query_scalar!(
            // language=PostgreSQL
            r#"insert into "function_history" (function_name, function_id, code,created_by,status,path,version,tag) values ($1,$2,$3,$4,$5,$6,$7,$8) returning function_id"#,
            function.function_name,
            function.function_id,
            function.code,
            function.created_by,
            function.status,
            function.path,
            function.version,
            tag
        )
        .fetch_one(db)
        .await?;
        Ok(id.to_string())
    }

    pub async fn page_function_history(
        &self,
        db: &PgPool,
        id:&Uuid,
        page: PageParams,
    ) -> Result<FnHistoryPageResponse> {
        let pagination = Pagination::build_from_request_query(page).count(1).build();
        let res = sqlx::query_as!(
            FunctionHistory,
            // language=PostgreSQL
            r#"select function_history_id,function_id,function_name,path,version,status,code,call_number,created_by,created_at,tag from "function_history" where function_id=$3 order by created_at desc limit $1 offset $2"#,
            pagination.limit,
            pagination.offset,
            id
        )
        .fetch_all(db)
        .await?;
        let total = sqlx::query_scalar!(
            // language=PostgreSQL
            r#"select count(*) from "function_history""#,
        )
        .fetch_one(db)
        .await?;
        let page_turn = PageTurnResponse::new(total.unwrap_or_default(), pagination.limit, res);
        Ok(page_turn)
    }
}
