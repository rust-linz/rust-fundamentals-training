# Exercise: Console Tic Tac Toe

## Introduction

In this exercise, you write a Tic Tac Toe game for two players that runs in the console. The whole game fits into a single file. You practice the basics of Rust: arrays, loops, functions, structs, reading from *stdin*, and writing to *stdout*.

## Requirements

* The app displays the board and asks for a location in the form *A1*..*C3*. The letter is the column, the digit is the row.
* Player *X* starts, then the players take turns.
* If the input is invalid or the square is already taken, the app prints a message and asks again.
* After each move, the app displays the board again. The game ends when a player has won or the board is full (draw).

The following sample output illustrates the concept. The sample solution also prints the parsed location using the `Debug` trait:

```txt
┌───┬───┬───┐
│   │   │   │
├───┼───┼───┤
│   │   │   │
├───┼───┼───┤
│   │   │   │
└───┴───┴───┘
Enter location (A1-C3): A1
Location { row: 0, col: 0 }
┌───┬───┬───┐
│ X │   │   │
├───┼───┼───┤
│   │   │   │
├───┼───┼───┤
│   │   │   │
└───┴───┴───┘
Enter location (A1-C3): B1
Location { row: 0, col: 1 }
┌───┬───┬───┐
│ X │ O │   │
├───┼───┼───┤
│   │   │   │
├───┼───┼───┤
│   │   │   │
└───┴───┴───┘

...

Enter location (A1-C3): A3
Location { row: 2, col: 0 }
┌───┬───┬───┐
│ X │ O │   │
├───┼───┼───┤
│ X │ O │   │
├───┼───┼───┤
│ X │   │   │
└───┴───┴───┘
X wins!
```

## Tips

Here is how you can store the board:

```rs
const SIZE: usize = 3;
let mut board = [[' '; SIZE]; SIZE];
```

You can read from *stdin* as follows. Note that `print!` does not flush the output, so you have to do that yourself before waiting for input:

```rs
use std::io::{stdin, stdout, Write};

print!("Enter location (A1-C3): ");
stdout().flush().unwrap();

let mut input = String::new();
stdin().read_line(&mut input).unwrap();
```

Keep the following in mind when processing the input:

* The input contains the line break (`\n` on Linux and macOS, `\r\n` on Windows). Use `trim` to get rid of it.
* `read_line` returns `0` when there is no more input (e.g. Ctrl+D). Don't ask again in that case, your program would never end.
* Byte literals like `b'A'` and `as_bytes` make it easy to calculate the column and row. Be careful with subtractions: `b'0' - b'1'` overflows.

## Advanced Versions

* Highlight the winning line using the [*colored* crate](https://docs.rs/colored/latest/colored/). Take a look at the function `print_colored_board` in the sample solution to get started.

## Sample Solution

The file [*src/main.rs*](src/main.rs) contains a sample solution. Compare it with your solution and discuss the differences and similarities.
