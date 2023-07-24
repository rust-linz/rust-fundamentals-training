# Guess The Number CLI Game

This is a simple console-based guessing game programmed in Rust. The aim of the game is to guess a randomly generated number within a certain number of attempts, both specified as command-line arguments when starting the game.

## Game Logic

Write a program that consists of the following steps:

1. **Creates a random number generator**: It initializes the random number generator which will be used to generate the target number to guess.

2. **Handles command-line arguments**: It collects the command-line arguments into a vector of strings. The program expects two arguments: the number of guesses a player has and the maximum possible number that could be guessed.

3. **Gameplay loop**: If the correct number of arguments is provided, the game loop begins. The loop runs for the number of guess attempts specified. In each iteration, the player's guess is read from standard input and compared with the random target number. Depending on whether the guess is lower or higher than the target, appropriate messages are printed to the console. If the guess is correct, a success message is printed and the loop breaks.

## Tipps

* Use the `rand` crate for generating random numbers
* Use `std::env` for handling command-line arguments
* Use `std::io` for interacting with the standard input/output streams

## Running the Code

To play the game, compile the program and run it with two numerical command-line arguments. The first argument is the number of guesses you have, and the second argument is the maximum possible number that could be guessed. Example:

```sh
cargo run 5 100
```

This command starts the game with 5 guesses, and the number to guess is a random number between 0 and 100. During the game, you'll need to input your guesses via the command line.

## Levels

### Level 1 - Basics

* Make assumptions for the input values and omit command line parameters.

### Level 2 - Command-line Arguments

* Add the ability to pass parameters via the command line.
* Remember to check the parameters with appropriate error messages.

### Level 3 - Unit Testing

* Structure your code so that you can test it well.
* Write at least one meaningful unit test ([brief guide](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html))
