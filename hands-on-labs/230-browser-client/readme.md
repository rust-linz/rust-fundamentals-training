# Browser Client

## Introduction

Now that we have a working API, we need a UI. As this training focusses on Rust, we keep the browser UI very simply. It just consists of a hand full of static HTML, JS, and CSS files. However, we want that our Rocket server serves the static files to the browser.

## Static File Serving

Our *main.rs* file already contains the code for serving static files from a folder named *public*. Try to find the corresponding line of code in *main.rs*.

## Adding Client

Create a new folder *public* in your *battleship_web_api* folder. Copy the following files into that folder and make yourself familiar with the code.

* [index.html](../999-final/battleship_web_api/public/index.html)
* [index.css](../999-final/battleship_web_api/public/index.css)
* [index.js](../999-final/battleship_web_api/public/index.js)

## Verify

Run your web API project with `cargo run` and use your favorite web browser to launch the UI. Inspect the network traffic to see communication from JS in the browser with Rust on the backend.
