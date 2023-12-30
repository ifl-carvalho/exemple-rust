# project-rust-axum-api

## Install

```shell
git clone git@github.com:ifl-carvalho/project-rust-axum-api.git

cd project-rust-axum-api

Copy .env.example to .env

docker-compose up

*** Before going forward remember to run Migrations. ***
```

## Migration

```shell
Make sure the .env is set up with correct DATABASE_URL

then:

cargo sqlx migrate run
```
