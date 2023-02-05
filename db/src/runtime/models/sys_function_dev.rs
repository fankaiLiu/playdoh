// create table "sys_function_dev" (
//     function_dev_id uuid primary key default uuid_generate_v1mc(),
//     function_name text collate "case_insensitive" unique not null,
//     function_id uuid unique not null,
//     status varchar(50) not null default 'draft',
//     code text collate "case_insensitive" not null,
//     call_number integer not null default 0,
//     created_by uuid not null,
//     created_at timestamptz not null default now(),
//     updated_by uuid default null,
//     updated_at timestamptz default null
// );

use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub function_name: String,
    pub function_id: Uuid,
    pub code: String,
    pub created_by :Uuid,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateReq {
    pub function_dev_id: Uuid,
    pub function_name: String,
    pub status: String,    
    pub code: String,
    pub updated_by :Uuid,
}

