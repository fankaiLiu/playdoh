use uuid::Uuid;

use crate::runtime::entities::function::Function;


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
#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub function_id: Uuid,
    pub function_name: String,
    pub status: String,
    pub code: String,
    pub created_by: Uuid,
    pub tag: Option<String>,
    pub path: String,
    pub version: i32,
}
impl From<Function> for AddReq  {
    fn from(value: Function) -> Self {
        Self {
            function_id: value.function_id,
            function_name: value.function_name,
            status: value.status,
            code: value.code,
            created_by: value.created_by,
            tag: None,
            path: value.path,
            version: value.version,
        }
    }
}
