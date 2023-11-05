-- Add migration scrpt here

create table staff (
  id bigserial primary key,
  customer_id bigint not null references customer,
  name varchar not null,
  phone varchar not null,
  pin int not null unique,
  fob int unique,
  created timestamptz not null default current_timestamp
);
