create table users
(
    id         serial
        constraint users_pk
            primary key,
    email      citext                              not null
        unique,
    username   citext                              not null
        unique,
    password   text                                not null,
    created_at timestamp default CURRENT_TIMESTAMP not null
);

alter table users
    owner to nometa_user;

