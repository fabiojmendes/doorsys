-- Add migration script here

create table customer (
  id bigserial primary key,
  name varchar not null,
  email varchar not null
);
