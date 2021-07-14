# Advanced topics

---

## Iterators and closures

```rust
let mut vec = Vec::<i32>::new();

for _ in 0..10 {
    let random_number = rand::thread_rng().gen_range(0..=50);
    if random_number % 3 != 0 {
        vec.push(random_number);
    }
}
```

Everything that implements an *iterator* can be used in a *for* loop

---

## Iterators and closures

Some types that implement iterators can also be used in a functional way:

```rust
let random_range: Vec<i32> = (0..10)
    .map(|_| rand::thread_rng().gen_range(0..=50))
    .filter(|el| el % 3 != 0)
    .collect::<Vec<i32>>();
```

`map` and `filter` take closures -> anonymous functions that are executed for each element.
Functional interfaces on basic types are zero cost abstractions and evaluated on compile time!

---

## iter vs into_iter

- The iterator returned by `into_iter` may yield any of `T`, `&T `or `&mut T`, depending on the context.
- The iterator returned by `iter` will `yield &T`, by convention.
- The iterator returned by `iter_mut` will yield `&mut T`, by convention.

---

## IntoIterator trait

```rust
pub trait IntoIterator 
where
    <Self::IntoIter as Iterator>::Item == Self::Item, 
{
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
```

---

## IntoIterator for Vec

```rust
impl<T> IntoIterator for Vec<T>
impl<'a, T> IntoIterator for &'a Vec<T>
impl<'a, T> IntoIterator for &'a mut Vec<T>
```

---

## The Iterator trait

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    ...
}
```

---

## Implement an Iterator

```rust
impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
```

---


## Implement an IntoIterator

```rust
impl IntoIterator for Lotto {
    type Item = <Vec<usize> as IntoIterator>::Item;
    type IntoIter = <Vec<usize> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.numbers.into_iter()
    }
}
```

---

## Wrapper types: Box<T>

`Box<T>` is
- an "owned pointer". It can hand out borrowed references, it is the only *owner*.
- a low cost abstraction for *dynamic allocation*
- at zero cost!

When the size is not known at compile time, use a `Box<T>` to defer the allocation at runtime.

```rust
struct Node {
    value: i32,
    next: Box<Node>
}
```

---

## Wrapper types: Rc<T>

Rc<T> is
- A *reference counted pointer*. Multiple owners, the data will be freed when all owners go out of scope
- Comes at a little runtime cost (internal reference counter)

---

## Wrapper types: Arc<T>

Arc<T> is a 
- An *atomic reference counted pointer*. This is thread-safe!
- Good in compbination with a `Mutex<T>`, `RwLock<T>`

---

## Wrapper types: Mutex<T>, RwLock<T>

- Shared data, `lock` will give access to it. 
- `RwLock` is efficient for multiple reads

---

## Threading

`thread.spawn` creates a new OS thread. `handle.join` starts executing

```rust
fn main() {
    let pairs = [(6, 45), (5, 50), (2, 12)];
    for (_take, _from) in pairs {
        let handle = thread::spawn(|| {
            let lotto = Lotto::new(6, 45);
            println!("{:?}", lotto);
        });

        handle.join().unwrap();
    }
}
```

---

## Threading

`move` closures give ownership of all referenced bindings to the closure. Now the thread can work with `take` and `from`


```rust
fn main() {
    let pairs = [(6, 45), (5, 50), (2, 12)];
    for (take, from) in pairs {
        let handle = thread::spawn(move || {
            let lotto = Lotto::new(take, from);
            println!("{:?}", lotto);
        });

        handle.join().unwrap();
    }
}
```

---

## Threading

`Mutex` allows mutually exclusive mutablility of values.
`Arc` is an *atomic reference counter* and points to the same data in memory. `Arc` is threadsafe!

```rust
let lottos = Mutex::new(Vec::<Lotto>::new());
let lottos = Arc::new(lottos);
let pairs = [(6, 45), (5, 50), (2, 12)];

for (take, from) in pairs {
    let lottos = Arc::clone(&lottos);
    let handle = thread::spawn(move || {
        let lotto = Lotto::new(take, from);
        lottos.lock().unwrap().push(lotto);
    });

    handle.join().unwrap();
}
```

In this example, we move the `Arc`, not the original value. Thanks to a `Mutex` we are able to edit the correct data!

---

## Dynamic dispath with trait objects

```rust

trait Greeter {
    fn greet(&self) -> String;
}

struct Person {
    name: String,
}

impl Greeter for Person {
    fn greet(&self) -> String {
        format!("Hey {}!", self.name)
    }
}

struct Dog;

impl Greeter for Dog {
    fn greet(&self) -> String {
        "Who is a good boy?".to_string()
    }
}
```

---

## Dynamic dispath with trait objects

Problem: Rust does not know at compile time if a `Person` or a `Dog` is being returned!

```rust
fn get_a_greeter(val: u8) -> impl Greeter {
    if val < 5 {
        Person {
            name: "unknown".to_string(),
        }
    } else {
        Dog {}
    }
}
```

Dog and Person are incompatible in Rust's eyes!

---

## Dynamic dispatch with trait objects

The `dyn` keyword annotates a *trait object*: A pointer to a trait
The `Box` ... again ... helps to give a statically available type!

```rust
fn get_a_greeter(val: u8) -> Box<dyn Greeter> {
    if val < 5 {
        Box::new(Person {
            name: "unknown".to_string(),
        })
    } else {
        Box::new(Dog {})
    }
}
```
