
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


use std::fmt::Display;

use serde::Deserialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub function_name: String,
    pub start_time: OffsetDateTime,
    pub end_time: OffsetDateTime,
    pub status: String,
    pub execution_user_id: Option<Uuid>,
    pub source: String,
    pub source_id: Uuid,
    pub result:Option<String>,
    pub console_log:Option<String>,
    pub console_error:Option<String>,
    pub duration_ms: i64,
    pub is_success: bool,
    pub arguments: Option<String>,
}
#[derive(Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct FnLog {
    pub function_log_id: i32,
    pub function_name: String,
    pub start_time: Option<String>,
    pub end_time: Option<OffsetDateTime>,
    pub status: String,
    pub execution_user_id: Option<Uuid>,
    pub source: Option<String>,
    pub source_id: Uuid,
    pub result: Option<String>,
    pub console_log: Option<String>,
    pub console_err: Option<String>,
    pub duration_ms: Option<i64>,
    pub is_success: bool,
    pub arguments: Option<String>,
}


impl AddReq {
    pub fn new(function_name:String,start_time:OffsetDateTime,source:Source,status:Status,user_id:Option<Uuid>,source_id:&Uuid,is_success:bool,
        arguments:String,result_log:String,console_log:Option<String>,console_error:Option<String>) -> Self {
        let now = OffsetDateTime::now_utc();
        AddReq {
            function_name,
            start_time,
            end_time: now,
            status: status.to_string(),
            execution_user_id: user_id,
            source: source.to_string(),
            source_id: *source_id,
            result: Some(result_log),
            console_log: console_log,
            console_error: console_error,
            duration_ms: (now-start_time).whole_milliseconds() as i64,
            is_success,
            arguments: Some(arguments),
        }
    }
}
#[derive(Deserialize, Clone, Debug)]
pub enum Source {
    Dev,
    Staging,
    Prod,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Dev => write!(f, "dev"),
            Source::Staging => write!(f, "staging"),
            Source::Prod => write!(f, "prod"),
        }
    }
}

pub enum Status {
    Running,
    Success,
    Failed,
    Timeout,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Running => write!(f, "running"),
            Status::Success => write!(f, "success"),
            Status::Failed => write!(f, "failed"),
            Status::Timeout => write!(f, "timeout"),
        }
    }
}
