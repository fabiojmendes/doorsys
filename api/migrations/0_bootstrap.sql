-- Add migration script here

create table admin (
  id bigserial primary key,
  name varchar not null,
  email varchar not null
)
