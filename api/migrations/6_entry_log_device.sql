-- Add migration scrpt here

alter table entry_log add column device_id bigint references device;
