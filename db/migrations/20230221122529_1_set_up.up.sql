-- Add up migration script here
-- This is a boilerplate migration file that we use in nearly every project.
-- It sets up database features that we use quite often.

-- As a style choice, we prefer not to write SQL in all uppercase as lowercase feels friendlier to the eyes.
-- It's nicer to read WHEN THE CODE ISN'T YELLING AT YOU ALL DAY.
-- It perhaps made sense back when code highlighting was not the norm and case was used to differentiate keywords
-- from non-keywords, but at this point it's purely from inertia.
-- The language itself is not case-sensitive except for quoted identifiers.
-- Whichever style you use, however, consistency should still be maintained.

-- This extension gives us `uuid_generate_v1mc()` which generates UUIDs that cluster better than `gen_random_uuid()`
-- while still being difficult to predict and enumerate.
-- Also, while unlikely, `gen_random_uuid()` can in theory produce collisions which can trigger spurious errors on
-- insertion, whereas it's much less likely with `uuid_generate_v1mc()`.
create extension if not exists "uuid-ossp";

-- We try to ensure every table has `created_at` and `updated_at` columns, which can help immensely with debugging
-- and auditing.
--
-- While `created_at` can just be `default now()`, setting `updated_at` on update requires a trigger which
-- is a lot of boilerplate. These two functions save us from writing that every time as instead we can just do
--
-- select trigger_updated_at('<table name>');
--
-- after a `CREATE TABLE`.
create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;

create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
    execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plpgsql;

-- Finally, this is a text collation that sorts text case-insensitively, useful for `UNIQUE` indexes
-- over things like usernames and emails, without needing to remember to do case-conversion.
create collation case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);

create table "sys_user" (
    user_id uuid primary key default uuid_generate_v1mc(),
    user_name text collate "case_insensitive" unique not null,
    user_nickname text collate "case_insensitive",
    email text collate "case_insensitive" unique not null,
    bio text not null default '',
    role_id uuid default null,
    dept_id uuid default null,
    remark text collate "case_insensitive" default null,
    is_admin int not null,
    phone_num varchar(20) default null,
    last_login_ip inet default null,
    last_login_time timestamptz default null,
    gender bigint not null,
    avatar text,
    status int,
    password_hash text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    deleted_at timestamptz
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT
    trigger_updated_at('"sys_user"');

create table "sys_oper_log" (
    log_id uuid primary key default uuid_generate_v1mc(),
    user_id uuid default null,
    time_id bigint not null default extract(
        epoch
        from
            now()
    ),
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

create table "sys_user_online" (
    id uuid primary key default uuid_generate_v1mc(),
    user_id uuid not null,
    token_id text collate "case_insensitive" not null,
    token_exp bigint not null,
    login_time timestamptz not null default now(),
    user_name text collate "case_insensitive" not null,
    dept_name text collate "case_insensitive" null,
    net text collate "case_insensitive" not null,
    ipaddr text collate "case_insensitive" not null,
    login_location text collate "case_insensitive" not null,
    device text collate "case_insensitive" not null,
    browser text collate "case_insensitive" not null,
    os text collate "case_insensitive" not null
);

alter table
    "sys_user_online"
add
    constraint "sys_user_online_user_id_fkey" foreign key (user_id) references "sys_user" (user_id) on delete cascade on update cascade;

create table "sys_menu" (
    id uuid primary key default uuid_generate_v1mc(),
    pid uuid Default NULL,
    path character varying(255) COLLATE pg_catalog."default" NOT NULL,
    menu_name character varying(255) COLLATE pg_catalog."default" NOT NULL,
    icon character varying(255) COLLATE pg_catalog."default" NOT NULL,
    menu_type character varying(255) COLLATE pg_catalog."default" NOT NULL,
    query character varying(255) COLLATE pg_catalog."default",
    order_sort integer NOT NULL,
    status character varying(255) COLLATE pg_catalog."default" NOT NULL,
    api character varying(255) COLLATE pg_catalog."default" NOT NULL,
    method character varying(255) COLLATE pg_catalog."default" NOT NULL,
    component character varying(255) COLLATE pg_catalog."default" NOT NULL,
    visible character varying(255) COLLATE pg_catalog."default" NOT NULL,
    is_cache character varying(255) COLLATE pg_catalog."default" NOT NULL,
    log_method character varying(255) COLLATE pg_catalog."default" NOT NULL,
    data_cache_method character varying(255) COLLATE pg_catalog."default" NOT NULL,
    is_frame character varying(255) COLLATE pg_catalog."default" NOT NULL,
    data_scope character varying(255) COLLATE pg_catalog."default" NOT NULL,
    i18n character varying(255) COLLATE pg_catalog."default",
    remark character varying(255) COLLATE pg_catalog."default" NOT NULL,
    created_at timestamptz not null default now(),
    updated_at timestamp without time zone,
    deleted_at timestamp without time zone,
    auth_type varchar(255) not null default 'read'
);

SELECT
    trigger_updated_at('"sys_menu"');

create table "sys_api_db" (
    api_id uuid NOT NULL,
    db text NOT NULL,
    primary key (api_id, db)
);

create table "sys_role" (
    role_id uuid primary key default uuid_generate_v1mc(),
    role_name varchar(255) not null,
    role_key varchar(255) not null,
    list_order integer not null,
    data_scope varchar(255) not null,
    status varchar(255) not null,
    remark varchar(255) not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

SELECT
    trigger_updated_at('"sys_role"');

create table "sys_user_role" (
    id uuid primary key default uuid_generate_v1mc(),
    role_id uuid not null,
    user_id uuid not null,
    created_by uuid not null,
    created_at timestamptz not null default now()
);

alter table
    "sys_user_role"
add
    constraint "sys_role_user_user_id_fkey" foreign key (user_id) references "sys_user" (user_id) on delete cascade on update cascade;

alter table
    "sys_user_role"
add
    constraint "sys_role_user_role_id_fkey" foreign key (role_id) references "sys_role" (role_id) on delete cascade on update cascade;

SELECT
    trigger_updated_at('"sys_user_role"');

create table "sys_role_api" (
    id uuid primary key default uuid_generate_v1mc(),
    role_id uuid not null,
    api varchar(255) not null,
    method varchar(255),
    auth_type varchar(255) not null default 'read',
    created_by uuid not null,
    created_at timestamptz not null default now()
);

alter table
    "sys_role_api"
add
    constraint "sys_role_api_role_id_fkey" foreign key (role_id) references "sys_role" (role_id) on delete cascade on update cascade;

create table "sys_dept" (
    dept_id uuid primary key default uuid_generate_v1mc(),
    parent_id uuid default null,
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
    deleted_at timestamptz
);

SELECT
    trigger_updated_at('"sys_dept"');

create table "sys_role_dept" (
    id uuid primary key default uuid_generate_v1mc(),
    role_id uuid not null,
    dept_id uuid not null,
    created_by uuid not null,
    created_at timestamptz not null default now()
);

alter table
    "sys_role_dept"
add
    constraint "sys_role_dept_role_id_fkey" foreign key (role_id) references "sys_role" (role_id) on delete cascade on update cascade;

alter table
    "sys_role_dept"
add
    constraint "sys_role_dept_dept_id_fkey" foreign key (dept_id) references "sys_dept" (dept_id) on delete cascade on update cascade;

create table "sys_login_log"(
    info_id uuid primary key default uuid_generate_v1mc(),
    login_name varchar(255) not null,
    ipaddr inet not null,
    net varchar(255) not null,
    login_location varchar(255) not null,
    browser varchar(255) not null,
    os varchar(255) not null,
    device varchar(255) not null,
    status varchar(255) not null,
    msg varchar(511) not null,
    login_time timestamptz not null default now(),
    module varchar(255) not null
);

create table "sys_post"(
    post_id uuid primary key default uuid_generate_v1mc(),
    post_code varchar(255) not null,
    post_name varchar(255) not null,
    post_sort int not null,
    status varchar(255) not null,
    remark text default null,
    created_by uuid default null,
    updated_by uuid default null,
    created_at timestamptz not null default now(),
    updated_at timestamptz default null,
    deleted_at timestamptz default null
);

SELECT
    trigger_updated_at('"sys_post"');

create table "sys_update_log" (
    id uuid primary key default uuid_generate_v1mc(),
    app_version varchar(255) not null,
    backend_version varchar(255) not null,
    title varchar(255) not null,
    content text not null,
    created_by uuid not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    deleted_at timestamptz default null
);

SELECT
    trigger_updated_at('"sys_update_log"');

CREATE TABLE "sys_user_dept" (
    id uuid primary key default uuid_generate_v1mc(),
    user_id uuid not null,
    dept_id uuid not null,
    created_by uuid not null,
    created_at timestamptz not null default now()
);

alter table
    "sys_user_dept"
add
    constraint "sys_user_dept_user_id_fkey" foreign key (user_id) references "sys_user" (user_id) on delete cascade on update cascade;

alter table
    "sys_user_dept"
add
    constraint "sys_user_dept_dept_id_fkey" foreign key (dept_id) references "sys_dept" (dept_id) on delete cascade on update cascade;

CREATE TABLE "sys_user_post" (
    id uuid primary key default uuid_generate_v1mc(),
    user_id uuid not null,
    post_id uuid not null,
    created_at timestamptz not null default now()
);

alter table
    "sys_user_post"
add
    constraint "sys_user_post_user_id_fkey" foreign key (user_id) references "sys_user" (user_id) on delete cascade on update cascade;

alter table
    "sys_user_post"
add
    constraint "sys_user_post_post_id_fkey" foreign key (post_id) references "sys_post" (post_id) on delete cascade on update cascade;

create table "function" (
    function_id uuid primary key default uuid_generate_v1mc(),
    function_name text collate "case_insensitive" unique not null,
    status varchar(50) not null default 'dev',
    code text collate "case_insensitive" not null,
    created_by uuid not null,
    created_at timestamptz not null default now(),
    updated_by uuid default null,
    call_number integer not null default 0,
    path text not null,
    version integer not null default 1,
    updated_at timestamptz default null
);

SELECT
    trigger_updated_at('"function"');

alter table
    "function"
add
    constraint "function_user_id_fkey" foreign key (created_by) references "sys_user" (user_id) on delete cascade on update cascade;

create table "function_history" (
    function_history_id uuid primary key default uuid_generate_v1mc(),
    function_id uuid not null,
    function_name text collate "case_insensitive" not null,
    status varchar(50) not null default 'dev',
    code text collate "case_insensitive" not null,
    created_by uuid not null,
    created_at timestamptz not null default now(),
    call_number integer not null default 0,
    path text not null,
    tag varchar(50) default null,
    version integer not null default 1
);

alter table
    "function_history"
add
    constraint "function_history_user_id_fkey" foreign key (created_by) references "sys_user" (user_id) on delete cascade on update cascade;

alter table
    "function_history"
add
    constraint "function_history_function_id_fkey" foreign key (function_id) references "function" (function_id) on delete cascade on update cascade;

create table function_log (
    function_log_id serial PRIMARY KEY,
    function_id uuid not null,
    function_name text NOT NULL,
    start_time timestamptz NOT NULL DEFAULT NOW(),
    end_time timestamptz,
    status varchar(50) NOT NULL,
    execution_user_id uuid default null,
    source varchar(50),
    source_id uuid not null,
    result_log text,
    duration_ms bigint,
    is_success boolean not null default false,
    arguments text
);

alter table
    "function_log"
add
    constraint "function_log_function_id_fkey" foreign key (function_id) references "function" (function_id) on delete cascade on update cascade;

create table oss_upload_history (
    id serial PRIMARY KEY,
    -- Primary key ID
    object_key varchar(255),
    -- Unique identifier of the file in OSS
    file_name varchar(255),
    -- Name of the file
    file_size bigint,
    -- Size of the file in bytes
    bucket_name varchar(255),
    -- Name of the storage space
    created_at timestamptz not null default now(),
    -- Upload time of the file
    created_by uuid not null,
    -- ID of the uploader
    description varchar(255),
    -- Description of the file
    status varchar(20),
    -- Status of the file, such as "deleted" or "normal"
    download_url varchar(255) -- Download URL of the file
);
