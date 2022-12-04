# Math Pyramid

## Introduction

A math pyramid is a frequently used tool for teaching math to elementary school children. The pyramid's base consists of a configurable number of random values between 0 and 9. Going up, each layer has one value less than the layer below. The values above the base are calculated by adding the left and right neighbours one level below.

Here is an example of such a math pyramid with a base consisting of five values:

```txt
                    ┌──────┐
                    │   70 │
                    └──────┘
               ┌──────┐   ┌──────┐
               │   29 │   │   41 │
               └──────┘   └──────┘
          ┌──────┐   ┌──────┐   ┌──────┐
          │   12 │   │   17 │   │   24 │
          └──────┘   └──────┘   └──────┘
     ┌──────┐   ┌──────┐   ┌──────┐   ┌──────┐
     │    8 │   │    4 │   │   13 │   │   11 │
     └──────┘   └──────┘   └──────┘   └──────┘
┌──────┐   ┌──────┐   ┌──────┐   ┌──────┐   ┌──────┐
│    8 │   │    0 │   │    4 │   │    9 │   │    2 │
└──────┘   └──────┘   └──────┘   └──────┘   └──────┘
```

Your job is to write a program that generates such math pyramids. Math teachers will use your program to generate quizzes for kids. Before handing out the quizzes, they will remove numbers from the pyramid and the kids have to fill in the blanks.

## Requirements

### Level 0: Basic Requirements

* Write a command-line application that generates a math pyramid as described above and print it on *stdout*.

* The application can receive the width of the base as an optional command line argument.
  * If no argument is given, a width of *5* is used as the default width.
  * If an argument is given, but it is not a number or not between 2 and 10 (including), the program should not generate the pyramid and print a proper error message on *stderr*.

The first version of the program should not care about nice formatting. Printing the raw values on the screen is sufficient. Here is an example output:

```txt
70
29 41
12 17 24
 8  4 13 11
 8  0  4  9  2
```

### Level 1: Modules

Extract different aspects of your application into separate modules.

* Create a module *args* for extracting your pyramid's base width from command-line arguments.
  * Create a file *args.rs*
  * Add `mod args;` to *main.rs*

* Similarly, create a module *pyramid* that contains the calculation logic for the math pyramid.
  * If you want, you can even put the calculation logic in a separate crate and add a folder-based dependency to *Cargo.toml*

### Level 2: Unit Tests

Add unit tests to your modules.

### Level 3: Output Formatting

Add nice output formatting as shown at the beginning of this document. Here are some constants that should make formatting easier:

```rs
pub const TOP_DECORATOR: &str = "┌──────┐";
pub const BOTTOM_DECORATOR: &str = "└──────┘";
pub const SEPARATOR_LENGTH: usize = 3;
pub const SEPARATOR_CHAR: u8 = b' '; // Note that this is a byte, not a character
pub const BAR: char = '│';
```

* Create a module *ascii_art* that contains helper functions and constants for output formatting.
  * Don't forget to add unit tests.

## Sample Solution

The folder [*math-pyramid*](math-pyramid) contains a sample solution. Compare it with your solution and discuss the differences and similarities.
