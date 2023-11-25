-- Add migration script here
create table config(
    initialised bool
);

insert into config(initialised) values(false);