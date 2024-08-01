-- Add migration scrpt here

alter table staff add constraint unique_staff_name_customer unique (name, customer_id);
