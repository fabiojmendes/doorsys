-- Add migration scrpt here

create type codetype as enum('pin', 'fob');

create table code (
  code varchar primary key,
  customer_id bigint not null references customer,
  code_type codetype not null
);
