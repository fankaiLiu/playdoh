use db::runtime::models::{sys_function_dev::*, function_log::Source};
use db::runtime::entities::sys_function_dev::*;
use db::runtime::models::function_log::{AddReq as FunctionLogAddReq, Status};


use sqlx::{Postgres, Pool};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::pagination::{PageParams, Pagination};
use crate::{Result, pagination::PageTurnResponse, apps::CONTEXT};
use playoh_runtime::jsruntime::run;
pub struct  RuntimeFuctionService{

}
pub type FnDevPageResponse = PageTurnResponse<FunctionDev>;

impl RuntimeFuctionService{
    pub fn new() -> Self {
        Self {}
    }
    pub async fn add_function_dev(&self,db: &Pool<Postgres>, req: AddReq) -> Result<String> {
        let function_id=uuid::Uuid::new_v4();
        let id = sqlx::query_scalar!(
            // language=PostgreSQL
            r#"insert into "sys_function_dev" (function_name, function_id, code,created_by ) values ($1,$2,$3,$4) returning function_dev_id"#,
            req.function_name,
            function_id,
            req.code,
            req.created_by,                
        )
        .fetch_one(db)
        .await?;
        Ok(id.to_string())
    }
    pub async fn get_function_dev(db: &Pool<Postgres>, function_id: &str) -> Result<Option<FunctionDev>> {
        let function_id=uuid::Uuid::parse_str(function_id)?;
        let res = sqlx::query_as!(
            FunctionDev,
            // language=PostgreSQL
            r#"select function_dev_id,function_name,function_id,status,code,call_number,created_by,created_at,updated_by,updated_at from "sys_function_dev" where function_id=$1"#,
            function_id,
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }
    pub async fn get_function_dev_by_name(db: &Pool<Postgres>, function_name: &str) -> Result<Option<FunctionDev>> {
        let res = sqlx::query_as!(
            FunctionDev,
            // language=PostgreSQL
            r#"select function_dev_id,function_name,function_id,status,code,call_number,created_by,created_at,updated_by,updated_at from "sys_function_dev" where function_name=$1"#,
            function_name,
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }
    pub async fn get_function_dev_by_id(db: &Pool<Postgres>, function_dev_id: &str) -> Result<Option<FunctionDev>> {
        let function_dev_id=Uuid::parse_str(function_dev_id)?;
        let res = sqlx::query_as!(
            FunctionDev,
            // language=PostgreSQL
            r#"select function_dev_id,function_name,function_id,status,code,call_number,created_by,created_at,updated_by,updated_at from "sys_function_dev" where function_dev_id=$1"#,
            function_dev_id,
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }
    pub async fn update_function_dev(&self,db: &Pool<Postgres>, function_dev: &UpdateReq,update_by:&Uuid)->Result<Option<FunctionDev>>{
        let function_dev_id= Uuid::parse_str(&function_dev.function_dev_id)?;
        let _res = sqlx::query_scalar!(
            // language=PostgreSQL
            r#"update "sys_function_dev" set function_name=$1,code=$2,updated_by=$3,updated_at=now() where function_dev_id=$4"#,
            function_dev.function_name,
            function_dev.code,
            update_by,
            function_dev_id,
        );
        return  Self::get_function_dev_by_id(db,&function_dev.function_dev_id).await;
    }
    pub async fn delete_function_dev(&self,db: &Pool<Postgres>, function_dev_id: &Uuid) -> Result<bool> {
        let res = sqlx::query_scalar!(
            // language=PostgreSQL
            r#"delete from "sys_function_dev" where function_dev_id=$1"#,
            function_dev_id,
        )
        .fetch_optional(db)
        .await?;
        Ok(res.is_some())
    }
    pub async fn page_function_dev(&self,db: &Pool<Postgres>, page:PageParams) -> Result<FnDevPageResponse> {
        let pagination = Pagination::build_from_request_query(page).count(1).build();
        let res = sqlx::query_as!(
            FunctionDev,
            // language=PostgreSQL
            r#"select function_dev_id,function_name,function_id,status,code,call_number,created_by,created_at,updated_by,updated_at from "sys_function_dev" order by created_at desc limit $1 offset $2"#,
            pagination.limit,
            pagination.offset,
        )
        .fetch_all(db)
        .await?;
        let total = sqlx::query_scalar!(
            // language=PostgreSQL
            r#"select count(*) from "sys_function_dev""#,
        )
        .fetch_one(db)
        .await?;
        let page_turn = PageTurnResponse::new(total.unwrap_or_default(),res );
        Ok(page_turn)
    }
    pub async fn run(db: &Pool<Postgres>, function_id: &Uuid,user_id:Option<Uuid>)->Result<String>
    {
        let record =sqlx::query!("select code ,function_name ,function_dev_id from sys_function_dev where function_id=$1",function_id).fetch_one(db).await?;
        let code=record.code;
        let now=OffsetDateTime::now_utc();
        let res=run(&code,"{}").await;
        let log=FunctionLogAddReq::new(record.function_name,now,Source::Dev,Status::Success,user_id,&record.function_dev_id.clone(),true,"{}".to_string(),res.clone());
        CONTEXT.runtime_function_log.add_function_log(db,log).await?;
        return Ok(res);
    }
}
