-- Add migration script here
create table users(
    user_id serial primary key,
    user_name varchar(255),
    user_email varchar(255),
    api_key varchar(255),
    enabled boolean
);