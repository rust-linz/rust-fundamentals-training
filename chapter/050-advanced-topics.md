# Advanced topics

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

## Async Rust

- Sometimes threads are way to heavy for concurrent operations
- Async execution of tasks is perfect if your application mostly waits for e.g. I/O
- Async becomes a first class citizien in Rust
- Right now you need some helper crates to get going
- `futures`, `async_std`

---

## Async functions

- Async functions implement the Future trait
- The default Future trait polls if the task is finished

```rust
pub trait Future {
    /// The type of value produced on completion.
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

---

## async / .await

- Async functions return a Future with a certain output
- Compare to `async` and `Promises` in JavaScript
- `.await` returns a result with a value of the async functions return type

```rust
// `foo()` returns a type that implements `Future<Output = u8>`.
// `foo().await` will result in a value of type `u8`.
async fn foo() -> u8 { 5 }
```

---

## File IO

- All we need to do is write `async` in front of it

```rust
async fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
```

---

## The right `.await`

- `.await` on each step lets you wait for the next step

```rust
async fn do_something_wrong() {
    let res_one = read_file("username.txt").await;
    let res_two = read_file("stairway.md").await;
    println!("{}", res_one.unwrap());
    println!("{}", res_two.unwrap());
}
```

## join!

- `join!` macro waits for all

```rust
async fn do_something() {
    let one = read_file("username.txt");
    let two = read_file("stairway.md");
    let (res_one, res_two) = join!(one, two);
    println!("{}", res_one.unwrap());
    println!("{}", res_two.unwrap());
}
```

---

## Where do you find Async Rust

- Everything on the Tokio Stack!
- Web Applications, File I/O
- Deno's event loop

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
