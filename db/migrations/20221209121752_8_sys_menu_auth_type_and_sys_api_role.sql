-- Add migration script here
--add auth_type to sys_menu
alter table
    "sys_menu"
add
    column auth_type varchar(255) not null default 'read';

CREATE TABLE "sys_role_api" (
    id uuid primary key default uuid_generate_v1mc(),
    role_id uuid not null,
    api varchar(255) not null,
    method varchar(255),
    auth_type varchar(255) not null,
    created_by uuid not null,
    created_at timestamptz not null default now()
);

alter table
    "sys_role_api"
add
    constraint "sys_role_api_role_id_fkey" foreign key (role_id) references "sys_role" (role_id) on delete cascade on update cascade;