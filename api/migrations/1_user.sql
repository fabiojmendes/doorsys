-- Add migration script here

create table users (
  id bigserial primary key,
  name varchar not null,
  email varchar not null
);
