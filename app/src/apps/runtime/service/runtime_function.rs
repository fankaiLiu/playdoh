use db::runtime::models::{function::*, function_log::Source};
use db::runtime::entities::function::*;
use db::runtime::models::function_log::{AddReq as FunctionLogAddReq, Status};


use sqlx::{Postgres, Pool};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::pagination::{PageParams, Pagination};
use crate::{Result, pagination::PageTurnResponse, apps::CONTEXT};
use playoh_runtime::jsruntime::run;
pub struct  RuntimeFuctionService{

}
pub type FnDevPageResponse = PageTurnResponse<Function>;

impl RuntimeFuctionService{
    pub fn new() -> Self {
        Self {}
    }
    pub async fn add_function_dev(&self,db: &Pool<Postgres>, req: AddReq,created_by:&Uuid) -> Result<String> {
        let function_id=uuid::Uuid::new_v4();
        let id = sqlx::query_scalar!(
            // language=PostgreSQL
            r#"insert into "function" (function_name, function_id, code,created_by ) values ($1,$2,$3,$4) returning function_id"#,
            req.function_name,
            function_id,
            req.code,
            created_by,                
        )
        .fetch_one(db)
        .await?;
        Ok(id.to_string())
    }
    pub async fn get_function_dev(db: &Pool<Postgres>, function_id: &str) -> Result<Option<Function>> {
        let function_id=uuid::Uuid::parse_str(function_id)?;
        let res = sqlx::query_as!(
            Function,
            // language=PostgreSQL
            r#"select function_id,function_name,path,version,status,code,call_number,created_by,created_at,updated_by,updated_at from "function" where function_id=$1"#,
            function_id,
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }
    pub async fn get_function_dev_by_name(db: &Pool<Postgres>, function_name: &str) -> Result<Option<Function>> {
        let res = sqlx::query_as!(
            Function,
            // language=PostgreSQL
            r#"select function_id,function_name,path,version,status,code,call_number,created_by,created_at,updated_by,updated_at from "function" where function_name=$1"#,
            function_name,
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }
    pub async fn get_function_dev_by_id(db: &Pool<Postgres>, function_id: &str) -> Result<Option<Function>> {
        let function_id=Uuid::parse_str(function_id)?;
        let res = sqlx::query_as!(
            Function,
            // language=PostgreSQL
            r#"select function_id,function_name,path,version,status,code,call_number,created_by,created_at,updated_by,updated_at from "function" where function_id=$1"#,
            function_id,
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }
    pub async fn update_function_dev(&self,db: &Pool<Postgres>, function_dev: &UpdateReq,update_by:&Uuid)->Result<Option<Function>>{
        let function_id= Uuid::parse_str(&function_dev.function_id)?;
        let _res = sqlx::query_scalar!(
            // language=PostgreSQL
            r#"update "function" set function_name=$1,code=$2,updated_by=$3,updated_at=now() where function_id=$4"#,
            function_dev.function_name,
            function_dev.code,
            update_by,
            function_id,
        );
        return  Self::get_function_dev_by_id(db,&function_dev.function_id).await;
    }
    pub async fn delete_function_dev(&self,db: &Pool<Postgres>, function_id: &Uuid) -> Result<bool> {
        let res = sqlx::query_scalar!(
            // language=PostgreSQL
            r#"delete from "function" where function_id=$1"#,
            function_id,
        )
        .fetch_optional(db)
        .await?;
        Ok(res.is_some())
    }
    pub async fn page_function_dev(&self,db: &Pool<Postgres>, page:PageParams) -> Result<FnDevPageResponse> {
        let pagination = Pagination::build_from_request_query(page).count(1).build();
        let res = sqlx::query_as!(
            Function,
            // language=PostgreSQL
            r#"select function_id,function_name,path,version,status,code,call_number,created_by,created_at,updated_by,updated_at from "function" order by created_at desc limit $1 offset $2"#,
            pagination.limit,
            pagination.offset,
        )
        .fetch_all(db)
        .await?;
        let total = sqlx::query_scalar!(
            // language=PostgreSQL
            r#"select count(*) from "function""#,
        )
        .fetch_one(db)
        .await?;
        let page_turn = PageTurnResponse::new(total.unwrap_or_default(),pagination.limit,res );
        Ok(page_turn)
    }
    pub async fn run(&self,db: &Pool<Postgres>, function_id: &Uuid,user_id:Option<Uuid>)->Result<String>
    {
        let record =sqlx::query!("select code ,function_name ,function_id from function where function_id=$1",function_id).fetch_one(db).await?;
        let code=record.code;
        let now=OffsetDateTime::now_utc();
        dbg!(&code);
        let res=run(&code,"{}").await;
        let log=FunctionLogAddReq::new(record.function_name,now,Source::Dev,Status::Success,user_id,&record.function_id.clone(),true,"{}".to_string(),res.clone());
        
        CONTEXT.runtime_function_log.add_function_log(db,log).await?;
        return Ok(res);
    }
}
