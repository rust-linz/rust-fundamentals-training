# Handling Errors

![Rust Linz](https://rust-linz.at/img/rust-linz-logo.svg)

---

> Make impossible states impossible

---

- In Rust, there is no `null` or `undefined`
- Bindings either have a value, or they don't.
- The `Option` enum helps!


---

```rust
let file_name = get_file_name_setting() // Option<String>

match file_name {
    Some(x) => println!("{}", x),
    None => println!("No filename, stored!")
};
```

---

```rust
let file_name = get_file_name_setting() // Option<String>

let file_handle = File::open(file_name.unwrap_or("username.txt"));

```

---

- In Rust, you don't have regular exception handling
- The Result type either has a result, or it has an error!

---

```rust
let f = File::open(path);

let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
};
```

---

Since constant error handling can get tedious, we can propagate error to 
the function above

```rust
let mut f = File::open(path)?;
```

