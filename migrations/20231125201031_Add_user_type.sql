-- Add migration script here

alter table users add column user_type varchar(20) not null default 'user';