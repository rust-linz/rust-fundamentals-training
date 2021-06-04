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


## Data types


## Structs and Enums

