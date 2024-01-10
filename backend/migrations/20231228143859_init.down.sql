-- Add down migration script here

DROP TABLE IF EXISTS produced_good_adjustments cascade;
DROP TABLE IF EXISTS produced_goods cascade;

DROP TABLE IF EXISTS sessions cascade;
DROP TABLE IF EXISTS users cascade;

DROP TABLE IF EXISTS products cascade;
DROP TABLE IF EXISTS measure_units cascade;
