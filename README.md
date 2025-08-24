# Plannify Admin API

This repo is a REST API for the Plannify admin project. It's developed in Rust using Axum framework.

## Repo Architecture

This repo is organized in the following way:
- `src/` contains the source code of the API
- `migrations/` contains the SQL migrations
- `.sqlx/` contains the SQLx migrations informations
- `.env.template` contains the environment variables template
- `Cargo.toml` contains the dependencies of the project
- `docker-compose.yml` contains the docker compose file
- `Dockerfile` contains the Dockerfile for the API
- `sqlx-cli.toml` contains the SQLx CLI configuration
- `sqlx-data.json` contains the SQLx data

## How to run the API

### Prerequisites

- Rust
- Cargo

### Run the API

You need to have a `.env` file in the root of the project. You can use the `.env.template` file as a template. Then you need to run the API external services:
```bash
docker compose up -d
```

In the docker compose, there is a service called `db-migrate` that will run the SQLx migrations. It will create the database and the tables automatically.

Then you can run the API:
```bash
cargo run
```

### Run the API with Docker

Soon.

## How to create a SQLx migrations

```bash
sqlx migrate add <migration-name> --source migrations/
```

## How to run the SQLx migrations

```bash
sqlx migrate run --source migrations/ --database-url $DATABASE_URL
```

`$DATABASE_URL` is the URL of the database. You can get it in the `.env` file.