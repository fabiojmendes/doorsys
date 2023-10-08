-- Add migration script here

create table entry_log (
  id bigserial primary key,
  staff_id bigint references staff,
  code varchar not null,
  created timestamptz not null default current_timestamp
);

create index entry_log_staff_idx on entry_log using btree(staff_id);
create index entry_log_created_idx on entry_log using btree(created);
