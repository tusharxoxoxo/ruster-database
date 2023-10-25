-- Your SQL goes here
create table videos(
    id serial not null,
    title character varying(255)  not null,
    description text not null,
    removed boolean not null,
    constraint videos_pkey primary key(id)
);
