-- Add migration script here


--生成表 pgsql
CREATE TABLE "sys_menu"
(
    id  uuid primary key default uuid_generate_v1mc(),
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
    deleted_at timestamp without time zone
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT
    trigger_updated_at('"sys_menu"');