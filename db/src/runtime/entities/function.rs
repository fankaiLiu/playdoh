
// create table "function" (
//     function_id uuid primary key default uuid_generate_v1mc(),
//     function_name text collate "case_insensitive" unique not null,
//     status varchar(50) not null default 'dev',
//     code text collate "case_insensitive" not null,
//     created_by uuid not null,
//     created_at timestamptz not null default now(),
//     updated_by uuid default null,
//     call_number integer not null default 0,
//     path text not null,
//     version integer not null default 1,
//     updated_at timestamptz default null
// );


use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize,Serialize, Clone, Debug,FromRow)]
pub struct Function {
    pub function_id: Uuid,
    pub function_name: String,
    pub status: String,
    pub code: String,
    pub created_by: Uuid,
    pub call_number: i32,
    pub created_at: OffsetDateTime,
    pub path: String,
    pub version: i32,
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<OffsetDateTime>,
}
