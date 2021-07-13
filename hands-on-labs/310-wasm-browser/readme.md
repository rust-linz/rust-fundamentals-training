# WebAssembly Browser

## Introduction

Now that we have the WASM package, we need a web client. The [Rust WASM book](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html#putting-it-into-a-web-page) contains a description how to setup a basic web client. In this exercise we do **not** follow this guide because it reference some older packages and uses JavaScript instead of TypeScript.

## Create Node Package

Create a subfolder *www* in the *battleship_wasm* package. Copy all files from [www](../999-final/battleship_wasm/www). Make yourself familiar with the code and/or discuss it with your Rust trainers.

## Dependencies

Take a look at the *package.json* file. Note how it references the WASM package.

Install the required Node packages by running `npm install` in the *www* folder.

## Verify

Run your *www* project with `npm start` and use your favorite web browser to launch the UI. Inspect the network traffic to see how the WASM module is loaded from the server.
