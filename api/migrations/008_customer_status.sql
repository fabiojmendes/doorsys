-- Add migration scrpt here

alter table customer add column active boolean not null default true;
