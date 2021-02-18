-- Your SQL goes here
create table cards (
    id integer primary key autoincrement not null,
    name text not null,
    color text,
    code text not null,
    user_id integer not null references users (id)
);

create table users (
    id integer primary key autoincrement not null,
    email text not null unique,
    name text not null,
    pass text not null
);

