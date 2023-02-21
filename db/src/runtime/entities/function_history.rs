
// create table "function_history" (
//     function_history_id uuid primary key default uuid_generate_v1mc(),
//     function_id uuid not null,
//     function_name text collate "case_insensitive" not null,
//     status varchar(50) not null default 'dev',
//     code text collate "case_insensitive" not null,
//     created_by uuid not null,
//     created_at timestamptz not null default now(),
//     call_number integer not null default 0,
//     path text not null,
//     tag varchar(50) default null,
//     version integer not null default 1
// );


use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize,FromRow)]
pub struct FunctionHistory {
    pub function_history_id: Uuid,
    pub function_id: Uuid,
    pub function_name: String,
    pub status: String,
    pub code: String,
    pub created_by: Uuid,
    pub created_at: OffsetDateTime,
    pub call_number: i32,
    pub path: String,
    pub tag: Option<String>,
    pub version: i32,
}