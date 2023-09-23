-- Add migration script here

create table code (
  id bigserial primary key,
  user_id bigint references users,
  code varchar unique not null
);
