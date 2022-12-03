-- Add migration script here
-- pub api_id: Uuid,
-- pub db: Uuid,
CREATE TABLE "sys_api_db"
(
    api_id  uuid  NOT NULL,
    db text NOT NULL,
    primary key (api_id, db)
);