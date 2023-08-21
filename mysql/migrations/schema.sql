drop table if exists post;
create table if not exists post (
    id CHAR(36) not null primary key,
    avater_id CHAR(36) not null,
    emoji_id CHAR(36) not null,
    place_id CHAR(36) not null,
    title text not null,
    comment text not null,
    visited_at datetime not null,
    created_at datetime not null,
    updated_at datetime not null 
);

drop table if exists emoji;
create table if not exists emoji (
    id CHAR(36) not null primary key,
    name text not null
);

drop table if exists avater;
create table if not exists avater (
    id CHAR(36) not null primary key,
    name text not null,
    account_id CHAR(36)
);

drop table if exists account;
create table if not exists account(
    id CHAR(36) not null,
    email text not null
)
