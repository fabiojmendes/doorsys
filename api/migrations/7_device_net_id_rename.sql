-- Add migration scrpt here

alter table device rename column mac_addr to net_id;
