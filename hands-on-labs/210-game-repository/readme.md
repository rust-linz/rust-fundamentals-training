# Game Repository

## Introduction

Axum can share state between request handlers ([see docs](https://docs.rs/axum/latest/axum/#sharing-state-with-handlers)), and real-world applications would typically use that to access a database. We want to keep things simple here because we are just learning Rust fundamentals. Therefore, we are going to implement a simple in-memory repository of Battleship games.

## *game_repository.rs*

Add the file *game_repository.rs* to the *battleship_web_api* package and declare it with `mod game_repository;` at the top of *main.rs*. Copy the content from [game_repository.rs](../999-final/battleship_web_api/src/game_repository.rs) or implement it yourself. Make yourself familiar with the code and/or discuss it with your Rust trainers. Note the recommended reading links at the beginning of the file.

The file also contains a mock version of the repository. We will use it for testing the web API in the next exercise.

## Verify

* Use `cargo build` to verify that your project compiles. The compiler might warn about unused code until we use the repository in the next exercise.

## Discussions

The repository uses a `std::sync::RwLock`, and our request handlers will be `async`. When is it ok to use a blocking lock in async code? What would change if a lock had to be held across an `.await`?
