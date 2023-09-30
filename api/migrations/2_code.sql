-- Add migration script here

-- create type CodeType as enum('pin', 'fob');

create table code (
  code varchar primary key,
  user_id bigint not null references users,
  code_type varchar not null
);
