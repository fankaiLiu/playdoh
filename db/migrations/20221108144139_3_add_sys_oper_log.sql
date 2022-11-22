-- Add migration script here
create table "sys_oper_log" (
    log_id uuid primary key default uuid_generate_v1mc(),
    user_id uuid default null,
    time_id bigint not null default extract(epoch from now()),
    title text collate "case_insensitive" not null,
    business_type text collate "case_insensitive" not null,
    method text collate "case_insensitive" not null,
    request_method text collate "case_insensitive" not null,
    operator_type text collate "case_insensitive" not null,
    oper_name text collate "case_insensitive" not null,
    dept_name text collate "case_insensitive" not null,
    oper_url text collate "case_insensitive" not null,
    oper_ip text collate "case_insensitive" not null,
    oper_location text collate "case_insensitive" not null,
    oper_param text collate "case_insensitive" not null,
    json_result text collate "case_insensitive" not null,
    path_param text collate "case_insensitive" not null,
    status text collate "case_insensitive" not null,
    error_msg text collate "case_insensitive" not null,
    duration bigint not null,
    oper_time timestamptz not null default now() 
);