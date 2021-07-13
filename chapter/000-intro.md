# Rust Fundamentals

## Getting Started

![Rust Linz](https://rust-linz.at/img/rust-linz-logo.svg)

---

## Rainer Stropek

- Microsoft MVP for Developer Technologies and Azure
- Microsoft Regional Director
- Co-founder [Rust Linz](https://rust-linz.at)
- [rainerstropek.me](https://rainerstropek.me)
- Twitter: [@rstropek](https://twitter.com/rstropek)

---

## Stefan Baumgartner

- Microsoft MVP in Developer Technologies
- Co-founder [Rust Linz](https://rust-linz.at)
- [fettblog.eu](https://fettblog.eu)
- [typescript-book.com](https://typescript-book.com)
- Twitter: [@ddprrt](https://twitter.com/ddprrt)

---

## Discussion: Why are you curious?

- What do you expect from Rust and this workshop?
  - Prior knowledge?
- What is appealing to you?
- What do you expect to be different from other languages you use?
- How would you convince someone who has never heard about Rust to look into it?

---

## Most 💕 language in Stackoverflow Survey 2016-2020

![Rust](./images/rustlove.png)

---

## Why Rust in Linux?

- No undefined behavior [...], including **memory safety** and the **absence of data races**.
- **Stricter type system** for further reduction of logic errors.
- A clear distinction between **safe and `unsafe`** code.
- **Featureful** language [...]
- Extensive freestanding **standard library** [...]
- Integrated out of the box **tooling** [...]

Overall, Rust is a language that has successfully leveraged decades of experience from system programming languages as well as functional ones [...] [(source)](https://lkml.org/lkml/2021/4/14/1023)

---

> Rust is an intriguing language. It closely resembles C++ in many ways, hitting all the right notes [...]. [...] it also has the potential to solve some of the most vexing issues that plague C++ projects [...].

[Microsoft](https://blogs.windows.com/windowsdeveloper/2020/04/30/rust-winrt-public-preview/)

---

> Rust provides memory safety guarantees [...] and runtime checks to ensure that memory accesses are valid. This safety is achieved while providing equivalent performance to C and C++.

[Google](https://security.googleblog.com/2021/04/rust-in-android-platform.html)

---

> For developers, Rust offers the performance of older languages like C++ with a heavier focus on code safety. Today, there are hundreds of developers at Facebook writing millions of lines of Rust code.

[Facebook](https://engineering.fb.com/2021/04/29/developer-tools/rust/)

---

## Real-world, production-grade projects

- [AWS Firecracker](https://firecracker-microvm.github.io/) - Virtualization on AWS Lambda and Fargate
- [Deno](https://deno.land) - A Node.js-like language runtime for JavaScript
- [Firefox](https://firefox.com) - A good deal of the new rendering engine is written in Rust
- Cloudflare - Core Edge Logic
- NPM - Parts of the registry architecture is written in Rust


---

## What's unique about Rust

- Rust is compiled to machine code (via LLVM) - native speed!
- A minimal runtime
- No gargabe collection, but a unique and effective memory management solution: Ownership
- Guaranteed memory safety
- "Debug at compile time" and "Hack without fear"
  
---

## Comparison to other languages


| Topic                  | Rust             | Go     | C#/Java | C/C++ |
|------------------------|------------------|--------|---------|-------|
| Memory management      | Ownership        | GC     | GC      | you   |
| Execution              | Native           | Native | VM      | Native|
| Null values            | x                |  ☑️     |  x/☑️    |  ☑️    |

---

## A feature-rich language

- Rust has influences from Alef, C#, C++, Cyclone, Erlang, Haskell, Limbo, Newsqueak, OCaml, Ruby, Scheme, Standard ML, Swift
- Zero-cost abstractions
- Cargo and crates for dependency management
- Same goals as Go, different approach
