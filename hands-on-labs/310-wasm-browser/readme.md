# WebAssembly Browser

## Introduction

Now that we have the WASM package, we need a web client. We build it with TypeScript and [Vite](https://vite.dev/). The [*vite-plugin-wasm*](https://github.com/Menci/vite-plugin-wasm) plugin lets us import the WASM package like any other module.

## Create Node Package

Create a subfolder *www* in the *battleship_wasm* package. Copy all files from [www](../999-final/battleship_wasm/www). Make yourself familiar with the code and/or discuss it with your Rust trainers.

## Dependencies

Take a look at the *package.json* file. Note how it references the WASM package.

Make sure that you have built the WASM package with `wasm-pack build` in the *battleship_wasm* folder. Then install the required Node packages by running `npm install` in the *www* folder.

## Verify

Run your *www* project with `npm start` and open the URL that Vite prints in your favorite web browser. Inspect the network traffic to see how the WASM module is loaded from the server.
