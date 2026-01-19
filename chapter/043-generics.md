# Generics

---

- Generics are very similar to other programming languages in syntax
- ... but they mostly work on a type system level (class system !== type system)

---

## Generic annotation

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4 };
}
```

---

## Trait bounds

```rust
fn print_example<T: Display>(val: T) {
    println!("{}", val);
}
```

- Trait bounds ensure that types are implementing certain traits

---

## Multiple trait bounds

```rust
fn print_example<T: Display + Copy>(val: T) {
    println!("{}", val);
}
```

- **Compound trait bound** - must satisify both `Display` and `Copy`

---

## Traitbounds with lifetimes

```rust
fn print_example<T: Display + Copy + 'a>(val: &T) {...}
```

- Must satisfy `Display` and `Copy`, must outlive `'a`

---

## where Syntax

```rust
fn print_example<T>(val :T)
where T: Display + Copy {

}
```