# Common Concepts

![Rust Linz](https://rust-linz.at/img/rust-linz-logo.svg)


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

- Variables are block-scoped

```rust
let main() {
    let x = 5;
    {
        let y = 99;
        println!("{}, {}", x, y);
    }
    println!("{}, {}", x, y); // 💥 Error!
}
```


- Values are dropped the moment they go out of scope

```rust
let main() {
    let x = 5;
    {
        let x = 99;
        println!("{}", x); // 99
    }
    println!("{}", x); // 5
}
```


## If expressions

- There are no statements, just expressions

```rust
if num == 5 {
    msg = "five";
}
```


```rust
if num == 5 {
    msg = "five";
} else if num == 4 {
    msg = "four";
} else {
    msg = "other";
}
```


```rust
let msg = if num == 5 {
   "five"
} else if num == 4 {
   "four"
} else {
   "other"
};
```

- Dropping the semicolon returns the value
- No `return`, as `return` exits a function
- Semicolon at the end of the entire expression to terminate the statement
- Braces are not optional


## Unconditional Loops

```rust
// Unconditional loops
loop {
    break;
}
```

```rust
'bob: loop {
    loop {
        loop {
            break 'bob;
        }
    }
}
```

```rust
'bob: loop {
    loop {
        loop {
            continue 'bob;
        }
    }
}
```


## Conditional loops


```rust
while some_condition() {
    // do stuff
}
```


## For loops 

```rust
for i in [1,2,3].iter() {
    println!("{}", i);
}
```

```rust
for i in 1..4 {
    println!("{}", i);
}
```

```rust
for i in 1..=3 {
    println!("{}", i);
}
```


## Functions

```rust
fn do_stuff(qty: f64, oz: f64) -> f64 {
    return qty * oz;
}
```

```rust
fn do_stuff(qty: f64, oz: f64) -> f64 {
    qty * oz
}
```

- Functions have a fixed argument list


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


## Tuple

```rust[1,2,3|5,6]
let address = (String::from("127.0.0.1"), 8080); // : (String, i32)
let ip = address.0;
let port = address.1;

// Destructuring
let (ip, port) = address;
```

- Maximum arity is 12
- Different types in a tuple


## Array

```rust
let ip = [127, 0, 0, 1]; //[i32; 4]
let buf = [0; 4];

let first = ip[0];
```

- Arrays are of fixed size


## Structs

Rust supports three struct types

```rust
// Classic struct with named fields
struct Point { x: f32, y: f32}

// Tuple struct with data types only
struct Matrix(f32, f32, f32, f32);

// Unit struct
struct Unit;
```


## Instantiate a struct

```rust
let point = Point { x: 0.23, y: -0.15 };
let matrix = Matrix(0, 0.2, -0.2, 1);

let x = 3;
let y = 3.12;

let point = Point { x, y };
```


## Implement functionality

With `impl` blocks you can define implementations on types.

```rust
impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
```


## Enums

Enums in Rust are more like algebraic union types. Each enum variant can have data to go along with it.

```rust
struct Individual {
    name: String,
}

enum Room {
    Occupied(Individual),
    Vacant,
}

let room = Room::Occupied(Individual { name: String::from("alice") } );
```