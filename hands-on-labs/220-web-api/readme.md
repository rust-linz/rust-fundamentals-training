# Web API

## Introduction

Now it is time to add the web API. If you have experience with web API development in other languages, building basic APIs with Axum will probably not be difficult for you. Pay particular attention to how handlers receive data through *extractors*, how the repository is shared between requests with `Arc`, and how `IntoResponse` turns our custom error type into HTTP responses. To make things more interesting, we have added API testing with mock objects (a mock version of our game repository).

## *main.rs*

Change the file *main.rs* in the *battleship_web_api* package. Copy the content from [main.rs](../999-final/battleship_web_api/src/main.rs) or implement it yourself based on the included unit tests. Make yourself familiar with the code and/or discuss it with your Rust trainers. Note the recommended reading links at the beginning of the file.

## Verify

* Use `cargo build` to verify that your project compiles.
* Use `cargo clippy` to verfiy that you follow Rust coding guidelines.
* Use `cargo test` to verify that all tests pass.

If you use VSCode's *REST Client*, you can use the sample requests in [requests.http](../999-final/battleship_web_api/requests.http) to test your API.
