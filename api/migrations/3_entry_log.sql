-- Add migration script here

create table entry_log (
  id bigserial primary key,
  code varchar not null references code,
  created timestamptz not null default current_timestamp
);

create index entry_log_code_idx on entry_log using btree(code);
create index entry_log_created_idx on entry_log using btree(created);
