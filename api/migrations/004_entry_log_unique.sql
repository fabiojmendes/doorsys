-- Add migration script here

alter table entry_log add constraint event_code_date unique (code, event_date);
