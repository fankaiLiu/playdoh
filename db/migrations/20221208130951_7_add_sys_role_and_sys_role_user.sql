    --pgsql
CREATE TABLE "sys_role"
(
    role_id  uuid primary key default uuid_generate_v1mc(),
    role_name varchar(255) not null,
    role_key varchar(255) not null,
    list_order integer not null,
    data_scope varchar(255) not null,
    status varchar(255) not null,
    remark varchar(255) not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);
-- And applying our `updated_at` trigger is as easy as this.
SELECT
    trigger_updated_at('"sys_role"');

CREATE TABLE "sys_role_user"
(
    id  uuid primary key default uuid_generate_v1mc(),
    role_id uuid not null,
    user_id uuid not null,
    created_by uuid not null,
    created_at timestamptz not null default now()
);
--添加 user表的user_id 的外键
alter table "sys_role_user" add constraint "sys_role_user_user_id_fkey" foreign key (user_id) references "user" (user_id) on delete cascade on update cascade;
--添加 role表的role_id 的外键
alter table "sys_role_user" add constraint "sys_role_user_role_id_fkey" foreign key (role_id) references "sys_role" (role_id) on delete cascade on update cascade;

-- And applying our `updated_at` trigger is as easy as this.
SELECT
    trigger_updated_at('"sys_role_user"');
