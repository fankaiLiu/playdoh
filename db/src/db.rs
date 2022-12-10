use std::borrow::BorrowMut;

use configs::CFG;
use log::LevelFilter;
use sqlx::postgres::{PgRow, PgArguments};
use sqlx::{ConnectOptions, Transaction, Postgres};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use tokio::sync::OnceCell;
use anyhow::anyhow;
//  Asynchronous initialization of the database
pub static DB: OnceCell<PgPool> = OnceCell::const_new();

pub async fn db_conn() -> PgPool {
    let mut pool_connection_options = PgConnectOptions::new();
    pool_connection_options.log_statements(LevelFilter::Trace);

    let db = PgPoolOptions::new()
        // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
        // Since we're using the default superuser we don't have to worry about this too much,
        // although we should leave some connections available for manual access.
        //
        // If you're deploying your application with multiple replicas, then the total
        // across all replicas should not exceed the Postgres connection limit.
        .max_connections(50)
        .connect(&CFG.database.link)
        .await
        .expect("数据库连接失败");
    // This embeds database migrations in the application binary so we can ensure the database
    // is migrated correctly on startup
    sqlx::migrate!().run(&db).await.expect("数据库迁移失败");
    db
}
pub struct TransactionManager{
    tran:Transaction<'static,Postgres>,
    rollback_only:bool
}

impl TransactionManager{
    pub fn new(tran:Transaction<'static,Postgres>) ->Self{
        TransactionManager {
            tran,
            rollback_only: false
        }
    }
    pub async fn rollback(self)->anyhow::Result<()>{
        self.tran.rollback().await?;
        Ok(())
    }
    pub async fn commit(self)->anyhow::Result<()>{
        if self.rollback_only {
            return Err(anyhow!("current transaction support rollback only"));
        }
        self.tran.commit().await?;
        Ok(())
    }
    pub fn transaction(&mut self) ->&mut Transaction<'static,Postgres>{
        self.tran.borrow_mut()
    }
    pub fn rollback_only(&mut self){
        self.rollback_only = true;
    }
}

pub enum SqlCommandExecutor<'db,'a> {
    UseTransaction(&'a mut TransactionManager),
    WithoutTransaction(&'db PgPool),
}

impl<'db,'a> SqlCommandExecutor<'db,'a> {

    pub async fn execute(&mut self, query: &str) -> anyhow::Result<u64> {
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result = sqlx::query(query).execute(tran_manager.transaction()).await?;
                Ok(result.rows_affected())
            } ,
            Self::WithoutTransaction(pool) => {
                let result = sqlx::query(query).execute(*pool).await?;
                Ok(result.rows_affected())
            },
        };
    }
    pub async fn execute_with(&mut self, query: &str,args:PgArguments) -> anyhow::Result<u64> {
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result = sqlx::query_with(query,args).execute(tran_manager.transaction()).await?;
                Ok(result.rows_affected())
            } ,
            Self::WithoutTransaction(pool) => {
                let result = sqlx::query_with(query,args).execute(*pool).await?;
                Ok(result.rows_affected())
            }
        };
    }
    pub async fn find_one<T>(&mut self, query: &str) -> anyhow::Result<T> where  T:  for<'r> sqlx::FromRow<'r, PgRow> ,T:Send,T:Unpin{
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result : T = sqlx::query_as(query)
                    .fetch_one(tran_manager.transaction()).await?;
                Ok(result)
            } ,
            Self::WithoutTransaction(pool) => {
                let result : T = sqlx::query_as(query)
                    .fetch_one(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn find_one_with<T>(&mut self, query: &str,args:PgArguments) -> anyhow::Result<T> where  T:  for<'r> sqlx::FromRow<'r, PgRow> ,T:Send,T:Unpin{
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result : T = sqlx::query_as_with(query,args)
                    .fetch_one(tran_manager.transaction()).await?;
                Ok(result)
            } ,
            Self::WithoutTransaction(pool) => {
                let result : T = sqlx::query_as_with(query,args)
                    .fetch_one(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn find_option<T>(&mut self, query: &str) -> anyhow::Result<Option<T>> where  T:  for<'r> sqlx::FromRow<'r, PgRow> ,T:Send,T:Unpin{
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result : Option<T> = sqlx::query_as(query)
                    .fetch_optional(tran_manager.transaction()).await?;
                Ok(result)
            } ,
            Self::WithoutTransaction(pool) => {
                let result : Option<T> = sqlx::query_as(query)
                    .fetch_optional(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn find_option_with<T>(&mut self, query: &str,args:PgArguments) -> anyhow::Result<Option<T>> where  T:  for<'r> sqlx::FromRow<'r, PgRow> ,T:Send,T:Unpin{
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result : Option<T> = sqlx::query_as_with(query,args)
                    .fetch_optional(tran_manager.transaction()).await?;
                Ok(result)
            } ,
            Self::WithoutTransaction(pool) => {
                let result : Option<T> = sqlx::query_as_with(query,args)
                    .fetch_optional(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn find_all<T>(&mut self, query: &str) -> anyhow::Result<Vec<T>> where  T:  for<'r> sqlx::FromRow<'r, PgRow> ,T:Send,T:Unpin{
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result : Vec<T> = sqlx::query_as(query)
                    .fetch_all(tran_manager.transaction()).await?;
                Ok(result)
            } ,
            Self::WithoutTransaction(pool) => {
                let result : Vec<T> = sqlx::query_as(query)
                    .fetch_all(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn find_all_with<T>(&mut self, query: &str,args:PgArguments) -> anyhow::Result<Vec<T>> where  T:  for<'r> sqlx::FromRow<'r, PgRow> ,T:Send,T:Unpin{
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result : Vec<T> = sqlx::query_as_with(query,args)
                    .fetch_all(tran_manager.transaction()).await?;
                Ok(result)
            } ,
            Self::WithoutTransaction(pool) => {
                let result : Vec<T> = sqlx::query_as_with(query,args)
                    .fetch_all(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn scalar_one<T>(&mut self, query: &str) -> anyhow::Result<T> where  T:  sqlx::Type<sqlx::Postgres> +  for<'r> sqlx::Decode<'r, sqlx::Postgres> ,T:Send,T:Unpin{
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result : T = sqlx::query_scalar(query)
                    .fetch_one(tran_manager.transaction()).await?;
                Ok(result)
            } ,
            Self::WithoutTransaction(pool) => {
                let result : T = sqlx::query_scalar(query)
                    .fetch_one(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn scalar_one_with<T>(&mut self, query: &str,args:PgArguments) -> anyhow::Result<T> where  T:  sqlx::Type<sqlx::Postgres> +  for<'r> sqlx::Decode<'r, sqlx::Postgres> ,T:Send,T:Unpin{
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result : T = sqlx::query_scalar_with(query,args)
                    .fetch_one(tran_manager.transaction()).await?;
                Ok(result)
            } ,
            Self::WithoutTransaction(pool) => {
                let result : T = sqlx::query_scalar_with(query,args)
                    .fetch_one(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn scalar_option<T>(&mut self, query: &str) -> anyhow::Result<Option<T>> where  T: sqlx::Type<sqlx::Postgres> +  for<'r> sqlx::Decode<'r, sqlx::Postgres> ,T:Send,T:Unpin{
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result : Option<T> = sqlx::query_scalar(query)
                    .fetch_optional(tran_manager.transaction()).await?;
                Ok(result)
            } ,
            Self::WithoutTransaction(pool) => {
                let result : Option<T> = sqlx::query_scalar(query)
                    .fetch_optional(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn scalar_option_with<T>(&mut self, query: &str,args:PgArguments) -> anyhow::Result<Option<T>> where  T:  sqlx::Type<sqlx::Postgres> +  for<'r> sqlx::Decode<'r, sqlx::Postgres> ,T:Send,T:Unpin{
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result : Option<T> = sqlx::query_scalar_with(query,args)
                    .fetch_optional(tran_manager.transaction()).await?;
                Ok(result)
            } ,
            Self::WithoutTransaction(pool) => {
                let result : Option<T> = sqlx::query_scalar_with(query,args)
                    .fetch_optional(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn scalar_all<T>(&mut self, query: &str) -> anyhow::Result<Vec<T>> where  T: sqlx::Type<sqlx::Postgres> +  for<'r> sqlx::Decode<'r, sqlx::Postgres> ,T:Send,T:Unpin{
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result : Vec<T> = sqlx::query_scalar(query)
                    .fetch_all(tran_manager.transaction()).await?;
                Ok(result)
            } ,
            Self::WithoutTransaction(pool) => {
                let result : Vec<T> = sqlx::query_scalar(query)
                    .fetch_all(*pool).await?;
                Ok(result)
            }
        };
    }
    pub async fn scalar_all_with<T>(&mut self, query: &str,args:PgArguments) -> anyhow::Result<Vec<T>> where  T: sqlx::Type<sqlx::Postgres> +  for<'r> sqlx::Decode<'r, sqlx::Postgres> ,T:Send,T:Unpin{
        return match self {
            Self::UseTransaction(ref mut tran_manager) =>{
                let result : Vec<T> = sqlx::query_scalar_with(query,args)
                    .fetch_all(tran_manager.transaction()).await?;
                Ok(result)
            },
            Self::WithoutTransaction(pool) => {
                let result : Vec<T> = sqlx::query_scalar_with(query,args)
                    .fetch_all(*pool).await?;
                Ok(result)
            }
        };
    }
}
