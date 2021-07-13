# Traits

![Rust Linz](https://rust-linz.at/img/rust-linz-logo.svg)

---

## What are traits

- Traits define shared behaviour between types
- Behaviour is defined in an abstract way
- Traits are somewhat similar to interfaces in other programming languages
- This does not mean Rust has traditional OO! There are differences

---

## Defining a trait

```rust
struct Project { ... }
struct MaintenanceHours { ... }

pub trait Billable {
    fn bill(&self) -> f32;
}
```

---

## Implementing a trait

```rust
impl Billable for MaintenanceHours {
    fn bill(&self) -> f32 {
        self.hours * self.rate
    }
}
```

---

## Default implementations

```rust
pub trait Billable {
    fn bill(&self) -> f32 {
        0f32
    }
}
```

---

## Coherence rule

> You either own the type or own the trait

- It's not possible to implement a trait you don't own for a type you don't own
- You can implement your traits for your types
- You can implement your traits for other types
- You can implement foreign traits for your types

---


```rust
impl fmt::Display for MaintenanceHours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} hours à {} USD", self.hours, self.rate)
    }
}
```

---


## Trait bounds

- Trait bounds allow a function to only accept types that implement a certain trait

```rust
fn print_bill(bill: &impl Billable) {}

fn print_bill<T: Billable>(bill: &T) {}
```
