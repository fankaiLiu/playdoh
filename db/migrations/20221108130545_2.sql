-- Add migration script here
--给user表增加一列删除时间
alter table "user" add column deleted_at timestamptz;