-- Add migration script here

create table "sys_oper_online" (
    id uuid primary key default uuid_generate_v1mc(),
    user_id uuid not null,
    token_id text collate "case_insensitive" not null,
    token_exp bigint not null,
    login_time timestamptz not null default now(),
    user_name text collate "case_insensitive" not null,
    dept_name text collate "case_insensitive"  null,
    net text collate "case_insensitive" not null,
    ipaddr text collate "case_insensitive" not null,
    login_location text collate "case_insensitive" not null,
    device text collate "case_insensitive" not null,
    browser text collate "case_insensitive" not null,
    os text collate "case_insensitive" not null
);

--添加 user表得user_id 的外键
alter table "sys_oper_online" add constraint "sys_oper_online_user_id_fkey" foreign key (user_id) references "user" (user_id) on delete cascade on update cascade;