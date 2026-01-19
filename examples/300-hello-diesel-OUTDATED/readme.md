# Hello Diesel

## Preparation

* Install SQLite client libraries:
  * `sudo apt-get install libsqlite3-dev`
* Install Diesel CLI
  * `cargo install diesel_cli --no-default-features --features sqlite`
* Set environment variable for Diesel CLI (and/or use *.env*)
  * `export DATABASE_URL=./mydb.db`

## Create DB Structure

```txt
diesel setup
diesel migration generate create_hero_types
diesel migration generate create_hero
diesel migration run
```

* Consider [*diesel_migrations*](https://docs.rs/crate/diesel_migrations/2.0.0) crate for production

