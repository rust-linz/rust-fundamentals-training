# Ownership and Borrowing

![Rust Linz](https://rust-linz.at/img/rust-linz-logo.svg)

---

## Memory unsafety in Systems Programming

- Use after free
- Double free
- Buffer overreads and overwrites
- Null pointers

---

## Ownership

- Exactly one owner of each piece of data
- When the owner goes out of scope, that data gets cleaned up
- Owner can transfer ownership or "borrow" data

---

## Ownership

```rust
fn main() {
    //      v--- Memory allocation
    let x = String::from("Hello world");
    //  ^--- Owner
    println!("{}", x);
} // <-- Owner goes out of scope, cleanup!
```

---

## Moving

```rust
fn main() {
    let x = String::from("Hello World");
    let y = x; // <-- Value moved here. y is the owner

    println!("{}", x);
                // ^-- borrow of moved value: `x` ERROR!
}
```

---

## Moving

```rust
fn main() {
    let person = Person { name: "alice" };
    print_person(person); // <-- GIVE ownership
    print_person(person); // ERROR!!!
}

            //  v-- TAKE Ownership
fn print_person(person: Person) {
    //...
}
```

---

## Moving

- There are no copy constructors
- `clone()` is explicit!

---

## Shared borrow

```rust
fn main() {
    let person = Person { name: "alice" };
    print_person(&person); // <-- Borrow person
    print_person(&person); // OK!
}

            //  v-- work with a reference
fn print_person(person: &Person) {
    //...
}
```

---

## Mutable borrow

```rust
fn main() {
    let mut person = Person { name: "alice" };
    change_name(&mut person); // <-- Borrow person
}

            // v-- work with a mutable reference
fn change_name(person: &mut Person) {
    //...
}
```

---

## Ownership and borrowing

| Type   | Ownership        | Alias? | Mutate? |
|--------|------------------|--------|---------|
| T      | Owned            |        |    ☑️    |
| &T     | Shared reference |   ☑️    |         |
| &mut T | Mutable reference|        |    ☑️    |


---

## Lifetimes

```rust
let y = {
    let x = String::from("hi");
    &x
//  ^-- borrowed value does not live long enough
}; //<-- x dropped here while still borrowed
```

---


```rust
let y = {                       // ------------+-- 'b
    let x = String::from("hi"); // ----+-- 'a  |
                                //     |       |
    &x                          // ----+       |
};                              //             |
                                //             |
println!("{}", y);              // ------------+
```

---

## Explicit lifetimes

```rust
fn first<'a>(v: &'a Vec<String>) -> &'a String {
    return &v[0]
}
```
