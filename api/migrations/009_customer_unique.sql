-- Add migration scrpt here

alter table customer add constraint unique_customer_name unique (name);
