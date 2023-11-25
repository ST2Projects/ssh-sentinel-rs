#!/usr/bin/env just --justfile

release:
  cargo build --release    

lint:
  cargo clippy

prepare:
    cargo sqlx prepare --database-url " postgres://postgres:postgres@localhost:15628/postgres

migrate:
    cargo sqlx migrate run --database-url "postgres://postgres:postgres@localhost:15628/postgres"