
// create table function_log (
//     function_log_id serial PRIMARY KEY,
//     function_name text NOT NULL,
//     start_time timestamptz NOT NULL DEFAULT NOW(),
//     end_time timestamptz,
//     status varchar(50) NOT NULL,
//     execution_user_id uuid default null,
//     source varchar(50),
//     source_id uuid not null ,
//     result_log text,
//     duration_ms bigint,
//     is_success boolean not null default false,
//     arguments text
//   );               


use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub function_name: String,
    pub start_time: OffsetDateTime,
    pub end_time: OffsetDateTime,
    pub status: String,
    pub execution_user_id: Uuid,
    pub source: String,
    pub source_id: Uuid,
    pub result_log: String,
    pub duration_ms: i64,
    pub is_success: bool,
    pub arguments: String,
}

