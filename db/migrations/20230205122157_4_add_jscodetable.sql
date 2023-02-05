-- Add migration script here
-- Add migration script here
--函数表
create table "sys_function_dev" (
    function_dev_id uuid primary key default uuid_generate_v1mc(),
    function_name text collate "case_insensitive" unique not null,
    function_id uuid unique not null,
    status varchar(50) not null default 'draft',
    code text collate "case_insensitive" not null,
    call_number integer not null default 0,
    created_by uuid not null,
    created_at timestamptz not null default now(),
    updated_by uuid default null,
    updated_at timestamptz default null
);
SELECT
    trigger_updated_at('"sys_function_dev"');

create table function_log (
  function_log_id serial PRIMARY KEY,
  function_name text NOT NULL,
  start_time timestamptz NOT NULL DEFAULT NOW(),
  end_time timestamptz,
  status varchar(50) NOT NULL,
  execution_user_id uuid default null,
  source varchar(50),
  source_id uuid not null ,
  result_log text,
  duration_ms bigint,
  is_success boolean not null default false,
  arguments text
);               