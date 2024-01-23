# 7 Segments Display Expercise

## Description

In this exercise, you have to implement an ASCII-based [7-segment display](https://en.wikipedia.org/wiki/Seven-segment_display). In the exercise, you train the following aspects of Rust:

* Basic, numeric data types
* Loops
* Bit operations
* Working with structs
* Working with vectors and tuples
* Creating functions
* Create a reusable library
* Creating a workspace
* Adding dependencies (crates.io and path-based, dev dependencies)
* Creating unit tests

Here is a sample output of the program:

```txt
     _   _       _   _   _   _   _   _
  |  _|  _| |_| |_  |_    | |_| |_| | |
  | |_   _|   |  _| |_|   | |_|  _| |_|

08:32:05 rainer@DESKTOP-B6V5TGG console_display ±|main ✗|→ 
```

If you are completely new to Rust, try to solve as much as possible on your own. If you are stuck, ask your trainer or take a look at the sample solution that you can find in the same folder as this _readme_ file.

## Tasks

### Setup the Project

1. Create an empty [*workspace*](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) in a new folder.
2. Create a new library project `seven_segments` in the workspace (`cargo new --lib seven_segments`).
3. Create a console app `console_display` in the workspace (`cargo new console_display`).
4. Add the library `seven_segments` as a dependency to the console app using a `path` dependency.

### Implement the Logic

1. Implement the following function in `seven_segments`:

    ```rs
    /// Convert a decimal digit to a 7-segment representation.
    /// 
    /// The resulting value is a byte where each bit represents
    /// a segment of the 7-segment display. The bits are ordered
    /// as follows (see also https://en.wikipedia.org/wiki/File:7_Segment_Display_with_Labeled_Segments.svg):
    /// 
    /// * bit 0 = segment A
    /// * bit 1 = segment B
    /// * ...
    /// 
    /// Therefore, the digit 0 is represented by the value 0b011_1111. The digit
    /// 9 would be 0b110_1111.
    /// 
    /// # Arguments
    /// 
    /// * `digit` - The decimal digit to convert.
    /// 
    /// # Panics
    /// 
    /// This function panics if the input is not a decimal digit (0-9).
    pub fn decimal_to_segments(digit: u8) -> u8 {
        // TODO: Implement this function.
    }
    ```

2. Test your implementation of `decimal_to_segments` with some unit tests.

3. Implement the following function in `seven_segments`:

    ```rs
    /// Generate characters to print for a 7-segment display.
    /// 
    /// The resulting vector contains tuples of the form (position, character).
    /// The position is a byte where the higher 4 bits represent the x-coordinate
    /// and the lower 4 bits represent the y-coordinate of the character.
    /// X and y coordinates are between 0 and 2.
    /// 
    /// Here are all the digits with their representation:
    /// 
    /// ```txt
    ///      _   _       _   _   _   _   _   _
    ///   |  _|  _| |_| |_  |_    | |_| |_| | |
    ///   | |_   _|   |  _| |_|   | |_|  _| |_|
    /// ```
    /// 
    /// # Arguments
    /// 
    /// * `byte` - The byte representing the 7-segment display.
    ///            This value is typically generated using the
    ///            `decimal_to_segments` function.
    /// 
    /// # Panics
    /// 
    /// This function panics if the input is not a valid 7-segment
    /// representation (i.e. if it is larger than 0b111_1111).
    pub fn draw_7_segments(byte: u8) -> Vec<(u8, char)> {
        // TODO: Implement this function.
    }
    ```

2. Test your implementation of `draw_7_segments` with some unit tests.

Here are some useful tips:

* Use Rust's [`panic!` macro](https://doc.rust-lang.org/std/macro.panic.html) to panic if the input is invalid.
* Use Rust's [`unreachable!` macro](https://doc.rust-lang.org/std/macro.unreachable.html) if you know that a certain code path is unreachable under normal circumstances.

### Bonus Exercise: _rstest_

If you event want to go further, create a [parameterized test using _rstest_](https://docs.rs/rstest/latest/rstest/#creating-parametrized-tests).

### Implement the Console App

Reference the [_crossterm_ crate](https://docs.rs/crossterm/latest/crossterm/). Make yourself familiar with the modules `cursor` and `terminal`. Here is an example to help you get started. It clears the terminal and hides the cursor:

```rust
let mut stdout = std::io::stdout();
execute!(stdout, crossterm::terminal::Clear(ClearType::All)).unwrap();
execute!(stdout, crossterm::cursor::Hide{}).unwrap();
```

Implement the `main` method. It should perform the following tasks:

* Clear the terminal and hide the cursor.
* Store the number to display in an `u32` variable called `number_to_display`.
* Determine the length of the number to display (i.e. the number of digits).
* Loop over all digits of the number to display.
  * Extract the digit from the number to display using `/`, `%` and `pow` (e.g. `10u32.pow((length_of_number - 1) - ix)`).
  * Convert the digit to a 7-segment representation using the functions `draw_7_segments` and `decimal_to_segments` from the `seven_segments` library.
  * Loop over all segments.
    * Extract x and y coordinates from the segment using [bit operations](https://togglebit.io/posts/rust-bitwise/).
    * Move the cursor to the corresponding position (e.g. `execute!(stdout, crossterm::cursor::MoveTo(<add your x pos here>, <add your y pos here>)).unwrap();`)
    * Write the character to the console (e.g. `write!(stdout, "{}", <add your char here>).unwrap();`).
* Move the cursor below your output and show the cursor again.
