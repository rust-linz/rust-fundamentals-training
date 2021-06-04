# Common Concepts


## Variables

```rust[2,3|5|7,8]
fn main() { // Entry point for Rust apps
  let a_number = 10; // inferred tye
  let a_boolean = true;

  let another_number: u16 = 11; // annotated type

  println!("The number is {}.", a_number);
  println!("The boolean is {}.", a_boolean);
}
```

- `let` creates a binding to a value
- Types can be annotated or inferred by usage
- The `println!` macro puts them on `stdout`

Note: Explain that macros are functions with a variable argument lists. Functions in Rust usually have a fixed argument list.


## Mutability

```rust[1,2,4|6,7]
let immutable_number = 10;
println!("The number is {}", immutable_number);

immutable_number = 11; // 💥 Error!

let mut mutable_number = 10;
mutable_number = 11;
```

- In Rust, bindings are immutable by default
- The `mut` keyword allows for mutations!


## Shadowing

```rust[1,2|4,5|7,8]
// The first binding is created with the name "number"
let number = 5;

// A different binding shadows the name "number" 
let number = number + 5; 

// Again, another new binding is created
let number = number * 2; 
```

- *Shadowing*: re-declare bindings to create new bindings.


## Scope


## Data types

```rust
let number: u32 = "42".parse().expect("Not a number!");
```

- The annotation `u32` defines what `"42"` is being parsed into.


## Numbers

| Length                    | Signed      | Unsigned | Float       |
| --------------------------| ----------- | -------- | ----------- |
| 8-bit                     | `i8`        | `u8`     |             |
| 16-bit                    | `i16`       | `u16`    |             |
| 32-bit                    | _**`i32`**_ | `u32`    | `f32`       |
| 64-bit                    | `i64`       | `u64`    | _**`f64`**_ |
| 128-bit                   | `i128`      | `u128`   |             |
| *depends on architecture* | `isize`     | `usize`  |             |


## Numbers

- Standard infer for integers is `i32`
- Standard infer for floating point is `f64`
- `isize` and `usize` is as long as defined by the architecture (32-bit, 64-bit)
- `u8` is commonly refered to as byte
  
```rust
let x = 5u16;
let y = 3.12f32;
```


## Integer literals

| Type           | Example    |
| -------------- | ---------- |
| Decimal        | 100_000    |
| Hex            | 0xdeadbeef |
| Octal          | 0o7753211  |
| Binary         | 0b11101111 |
| Byte (u8 only) | b'A'       |


## Operations

All primitive types support mathematical operations

```rust
fn main() {
    // Addition
    println!("1 + 2 = {}", 1u32 + 2);
    // Subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // ^ Try changing `1i32` to `1u32` to see why the type is important
    // Integer Division
    println!("9 / 2 = {}", 9u32 / 2);
    // Float Division
    println!("9 / 2 = {}", 9.0 / 2.0);
    // Multiplication
    println!("3 * 6 = {}", 3 * 6)
}
```


## Booleans

Booleans in Rust are represented by the type `bool` and have two possible values: `true` or `false`.

```rust
fn main() {
    let is_bigger = 1 > 4;
    println!("{}", is_bigger);  // prints "false"
}
```


## Structs and Enums


## Char

`char` represent a single unicode scalar type!

```rust
let my_letter = 'a';
let ferris = '🦀';
```

They're pretty useless. Strings are UTF-8, characters are not. So they're not used internally.

## Strings

- There are 6 types of String in the Rust standard library
- Usually we care about 2 types


## String slices

- The first is a string slice `str`
- We mostly see it as a *borrowed* string slice `&str`

```rust
let msg = "Hello World" // : &str
```

- Literal strings are always borrowed string slices 


## String

- The other is `String`
- Data in `&str` can't be modified, data in `String` can

```rust
let msg = "Hello World".to_string(); // : String
let msg = String::from("Hello World");
```


## Details

- `&str` is made of a pointer to some bytes and a length
- `String` is made of a pointer to some bytes, length, and capacity
- Both are valid UTF-8
- Strings can't be indexed by character positions

