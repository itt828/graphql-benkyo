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

