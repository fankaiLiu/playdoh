-- Add migration script here
CREATE TABLE "sys_dept" (
    dept_id uuid primary key default uuid_generate_v1mc(),
    parent_id uuid not null,
    dept_name varchar(255) not null,
    order_num integer not null,
    leader varchar(255) default null,
    phone varchar(20) default null,
    email varchar(255) default null,
    status varchar(255) not null,
    created_by uuid not null,
    updated_by uuid not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    deleted_at timestamptz default null
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT
    trigger_updated_at('"sys_dept"');

CREATE TABLE "sys_role_dept" (
    id uuid primary key default uuid_generate_v1mc(),
    role_id uuid not null,
    dept_id uuid not null,
    created_by uuid not null,
    created_at timestamptz not null default now()
);