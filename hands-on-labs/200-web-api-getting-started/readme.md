# Battleship Web API

## Introduction

In the following exercises, we want to build a web API for playing Battleship games including a simplified web client (just JavaScript, no fancy SPA framework).

You can choose from a [huge amount of frameworks for web programming in Rust](https://github.com/rust-unofficial/awesome-rust#web-programming). In this exercise, we are going to use the [Rocket](https://rocket.rs/) framework.

## Create Package

Create a new binary target using Cargo: `cargo new --bin battleship_web_api`. Add the new package to the previously created workspace.

Use `cargo build` to verify that your project compiles.

## Dependencies

Copy the dependencies into the library's *Cargo.toml* from [Cargo.toml](../999-final/battleship_web_api/Cargo.toml). Lookup all the referenced crates on [crates.io](https://crates.io/) and find out what they are used for. You do not need to understand all the details. Overview knowledge is sufficient. Note how we reference our game logic in *battleship_game_logic*.

**Note:** At the time of writing, Rocket 0.5 has not been released yet. Therefore, this exercise is built on the current release candidate. Use the released version of Rocket 0.5 if it is already available when you do this exercise.

## Hello World

Copy the *Hello World* example from [Rocket's documentation](https://rocket.rs/v0.5-rc/guide/getting-started/#hello-world) into your *main.rs* file. Start the web server with `cargo run` and verify that you get a proper response.

For testing the web API, we recommend one of the following VSCode extensions:

* [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client)
* [Thunder Client](https://marketplace.visualstudio.com/items?itemName=rangav.vscode-thunder-client)

Note that this repository contains sample requests for the *REST Client* in [requests.http](../999-final/battleship_web_api/requests.http).
