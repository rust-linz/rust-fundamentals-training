# Slides for Rust fundamentals training

## Introduction

This folder contains slides for the *Rust Fundamentals Training*.

## Usage

### Folders

* The root folder should only contain settings files and this *readme.md* file.
* [*favicon*](favicon) contains images and settings files for icons.
* Your slide content should go in [*src*](src).

### VSCode

This folder contains [recommended VSCode extensions](.vscode/extensions.json). We recommend to install them to get the best editing experience.

## Building

To build the slides, run `npm run build`. You will get the built slide deck in the *dist* folder. Copy the *dist* folder to any static web server (e.g. GitHub) to host the slides.

If you want to build slides in *watch* mode, run `npm run watch-build`.

`npm run clean` will delete the *dist* folder and therefore gives you a clean environment for your next build if you need that.

Do you miss anything (e.g. a *reveal.js* plugin, some files from *src* are not copied)? Customize the build process in the scripts in [*util*](util).

## View Slides Locally

To start a webserver serving and watching the *dist* folder, run `npm run watch-start`.

If you want to watch source files and have a local webserver for auto-refresh for the slides, run `npm start`. This script runs the scripts `watch-build` and `watch-start` in parallel.
