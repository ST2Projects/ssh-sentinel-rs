-- Add migration script here
create table users(
    user_id serial primary key,
    user_name varchar(255),
    user_email varchar(255),
    api_key varchar(255),
    enabled boolean
);

insert into users(user_name, user_email, api_key, enabled) values ('admin', 'admin@st2projects.com', 'test-key', true);