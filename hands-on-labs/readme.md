# Battleship Hands-on-Lab

## Introduction

In this hands-on-lab, you are going to build various flavors of the classical [Battleship game](https://en.wikipedia.org/wiki/Battleship_(game)). First, we build a reusable library for the game's business logic. In that phase of the lab, you will learn and practice language fundamentals, use Rust tools, and write automated unit tests.

Next, we will implement a CLI (*Command Line Interface*) for our game. You will learn about useful libraries for building such command-line tools.

The third step is building a web API for a browser-based version of our game. We will use the *Axum* framework for that.

Last but not least we will create a WebAssembly (WASM) version of Battleship that runs our previously created game logic in the browser.

## Prerequisites

* [Rust](https://www.rust-lang.org/tools/install)
* [Visual Studio Code](https://code.visualstudio.com) with the extensions *rust-analyzer* und *CodeLLDB*
* [Node.js](https://nodejs.org/en/) (for web UI)
* [wasm-pack](https://drager.github.io/wasm-pack/) (for the WebAssembly exercises)

## Exercising

Ideally, you work on the exercises in this lab in a classroom with trainers. If you cannot attend one of our trainings, feel free to work on the sample yourself.

Depending on your existing coding skills, you can approach the examples in different ways:

### Level 1 - Limited Coding Skills

You are not a professional developers. You have written code in the past, but you are not deep into coding anymore. Your are more interested in the concepts.

In this case, we recommend that you clone this repository, read the code, try to get it running on your machine, and focus on gathering conceptual knowledge.

### Level 2 - No Rust Experience

You are a professional developer, but you have no prior Rust experience.

In this case, we recommend that you start with an empty folder and build the example step-by-step by copying the relevant code pieces from our sample. Make yourself familiar with the code, make sure that everything runs on your machine. Feel free to experiment by adding additional features to the sample.

### Level 3 - Existing Rust Experience

You have some basic Rust experience.

For you, we recommend that you copy just the automated tests and user interfaces (HTML, JS/TS, CSS). Try to add the data structures and logic on your own.

## Discussions

At the end of each exercise, discuss what you have learned. Here are some questions that can get you started:

* Did you apply Rust concepts or tools that reminded you of similar concepts from other programming languages you know? How is Rust different?
* What surprised you, positively and negatively?
* What concepts did you find particularly hard to understand?

## Additional Exercises

The folders with three-digit numbers contain the steps of the Battleship lab described above, and *999-final* contains its reference solution. The folders with four-digit numbers contain additional exercises. Each of them is independent of Battleship and of the other exercises, and comes with its own instructions and sample solution.

| Exercise | Description |
| -------- | ----------- |
| [Console Tic Tac Toe](1000-console-ttt/readme.md) | Two-player game in a single file for practicing arrays, loops, functions, structs, and console input and output |
| [Math Pyramid](1010-math-pyramid/readme.md) | Program that generates math pyramids as quizzes for school children |
| [Tic Tac Toe](1020-tic-tac-toe/readme.md) | Game built level by level, starting with data types for the square and board content |
| [Simple Tic Tac Toe](1021-simple-ttt/readme.md) | Console-based game that is described by its requirements only |
| [7 Segments Display](1030-7-segments/readme.md) | ASCII-based 7-segment display for practicing numeric types, bit operations, libraries, workspaces, and unit tests |
| [Rusty Nerdle](1040-rusty-nerdle/readme.md) | Number guessing game similar to *nerdle* that runs in the browser with WebAssembly |
