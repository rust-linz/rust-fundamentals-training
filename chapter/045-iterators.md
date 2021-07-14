# Iterators

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
