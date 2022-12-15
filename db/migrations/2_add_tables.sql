create table "sys_user" (
    user_id uuid primary key default uuid_generate_v1mc(),
    user_name text collate "case_insensitive" unique not null,
    user_nickname text collate "case_insensitive",
    email text collate "case_insensitive" unique not null,
    bio text not null default '',
    role_id uuid not null,
    dept_id uuid not null,
    remark text collate "case_insensitive" default null,
    is_admin int not null,
    phone_num varchar(20) default null,
    last_login_ip inet default null,
    last_login_time timestamptz default null,
    gender bigint not null,
    avatar text,
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