-- Add migration scrpt here

create table device (
  id bigserial primary key,
  name varchar not null,
  mac_addr varchar not null unique
);
