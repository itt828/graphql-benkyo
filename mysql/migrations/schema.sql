drop table if exists `blog`;

create table if not exists `blog` (
    `id` CHAR(36) not null primary key,
    `title` text not null,
    `content` text not null,
    `created_at` datetime not null default current_timestamp
);