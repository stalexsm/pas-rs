-- Add up migration script here

drop table if exists users cascade;

create table users (
    id bigserial primary key,
    role varchar(255) not null,
    email varchar(1000) not null unique,
    fio varchar(1000) null,
    passwd varchar(1000) null,
    blocked boolean not null default 'f',
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone null
);

create index on users (role);


drop table if exists sessions cascade;

create table sessions (
    id uuid primary key,
    user_id bigint not null references users(id) on delete cascade,
    expires_at timestamp with time zone not null,
    created_at timestamp with time zone not null default now()
);

create index on sessions (user_id);


drop table if exists measure_units cascade;

create table measure_units (
    id bigserial primary key,
    name varchar(255) not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone null
);


drop table if exists products cascade;

create table products (
    id bigserial primary key,
    measure_unit_id bigint not null references measure_units(id) on delete cascade,
    name varchar(255) not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone null
);

create index on products (measure_unit_id);


drop table if exists produced_goods cascade;

create table produced_goods (
    id bigserial primary key,
    user_id bigint not null references users(id) on delete cascade,
    product_id bigint not null references products(id) on delete cascade,
    cnt bigint not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone null
);

create index on produced_goods (user_id);
create index on produced_goods (product_id);


drop table if exists produced_good_adjustments cascade;

create table produced_good_adjustments (
    id bigserial primary key,
    user_id bigint not null references users(id) on delete cascade,
    produced_good_id bigint not null references produced_goods(id) on delete cascade,
    cnt bigint not null,
    created_at timestamp with time zone not null default now()
);


create index on produced_good_adjustments (user_id);
create index on produced_good_adjustments (produced_good_id);


insert into users (role, email, fio, passwd) values ('Admin', 'stalex.info@yandex.ru', 'А.С (Разработчик)', '$2b$12$o1Mjf.Uhbjye1tb2gRR82.5NOA/flndnWdMfn.i5YZpGZvq4pdL4i');
