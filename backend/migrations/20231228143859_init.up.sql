-- Add up migration script here

DROP TABLE IF EXISTS organizations CASCADE;

CREATE TABLE organizations (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(1000) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NULL
);

DROP TABLE IF EXISTS users CASCADE;

CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    organization_id BIGINT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    role VARCHAR(255) NOT NULL,
    email VARCHAR(1000) NOT NULL UNIQUE,
    fio VARCHAR(1000) NOT NULL,
    passwd VARCHAR(1000) NULL,
    blocked BOOLEAN NOT NULL DEFAULT 'F',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NULL
);

CREATE INDEX ON users (organization_id);
CREATE INDEX ON users (role);


DROP TABLE IF EXISTS sessions CASCADE;

CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX ON sessions (user_id);


DROP TABLE IF EXISTS measure_units CASCADE;

CREATE TABLE measure_units (
    id BIGSERIAL PRIMARY KEY,
    organization_id BIGINT NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NULL
);

CREATE INDEX ON measure_units (organization_id);


DROP TABLE IF EXISTS products CASCADE;

CREATE TABLE products (
    id BIGSERIAL PRIMARY KEY,
    organization_id BIGINT NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    measure_unit_id BIGINT NOT NULL REFERENCES measure_units (id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NULL
);

CREATE INDEX ON products (organization_id);
CREATE INDEX ON products (measure_unit_id);


DROP TABLE IF EXISTS produced_goods CASCADE;

CREATE TABLE produced_goods (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    product_id BIGINT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    cnt bigint NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NULL
);

CREATE INDEX ON produced_goods (user_id);
CREATE INDEX ON produced_goods (product_id);


DROP TABLE IF EXISTS produced_good_adjustments CASCADE;

CREATE TABLE produced_good_adjustments (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    produced_good_id BIGINT NOT NULL REFERENCES produced_goods(id) ON DELETE CASCADE,
    cnt bigint NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);


CREATE INDEX ON produced_good_adjustments (user_id);
CREATE INDEX ON produced_good_adjustments (produced_good_id);


INSERT INTO users (role, email, fio, passwd) VALUES ('Developer', 'stalex.info@yandex.ru', 'А.С (Разработчик)', '$2b$12$o1Mjf.Uhbjye1tb2gRR82.5NOA/flndnWdMfn.i5YZpGZvq4pdL4i');
