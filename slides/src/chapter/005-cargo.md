# Cargo

## The Rust Package Manager

![Rust Linz](https://rust-linz.at/img/rust-linz-logo.svg)

---

## Material

* [MIT licensed](LICENSE)
  * Use it, share it, build on it
* [Slides in Markdown](cargo.md)
* [Samples](samples) with hands-on lab descriptions
  * Use it for your own practicing
  * Use it in hackathons, trainings, etc.
* Contributions are welcome

---

# Jumpstart

For Rust Beginners ([read more...](https://doc.rust-lang.org/cargo/guide/index.html))

---

## Purpose of *Cargo*

* Dependency <!-- .element: class="fragment" --> management
* Invoke <!-- .element: class="fragment" --> Rust compiler

```bash [1|3-6|7,9]
cargo new hello_world --bin # --bin for program, --lib for library

cd hello_world/
cargo build # implicitly uses profile `dev`
./target/debug/hello_world

# Compile with `release` profile
# More about profiles at https://doc.rust-lang.org/cargo/reference/profiles.html
cargo build --release
./target/release/hello_world
```
<!-- .element: class="fragment" -->

---

## Important Cargo Commands

| Command                                                                       | Description                                                |
| ----------------------------------------------------------------------------- | ---------------------------------------------------------- |
| [`cargo new`](https://doc.rust-lang.org/cargo/commands/cargo-new.html)        | Create a new Cargo package                                 |
| [`cargo search`](https://doc.rust-lang.org/cargo/commands/cargo-search.html)  | Search packages in *crates.io*                             |
| [`cargo tree`](https://doc.rust-lang.org/cargo/commands/cargo-tree.html)      | Display a tree visualization of a dependency graph         |
| [`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html)    | Compile the current package                                |
| [`cargo run`](https://doc.rust-lang.org/cargo/commands/cargo-run.html)        | Run the current package                                    |
| [`cargo fetch`](https://doc.rust-lang.org/cargo/commands/cargo-fetch.html)    | Fetch dependencies from network (prepare for offline work) |

---

## [Project File Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)

```txt
.
├── Cargo.lock
├── Cargo.toml
├── src/ 
│     Source code
├── benches/
│     Benchmarks
├── examples/
│     Example code
└── tests/ (integration tests)
      Integration tests
```

---

## *Cargo.toml*

* Similar <!-- .element: class="fragment" --> to npm's *package.json*
* Has <!-- .element: class="fragment" --> to be called *Cargo.toml* (uppercase *C*)
  * [*Tom's Obvious, Minimal Language*, short *TOML*](https://github.com/toml-lang/toml)
* Built <!-- .element: class="fragment" --> on [Semantic Versioning](https://semver.org/)
* Package  <!-- .element: class="fragment" --> metadata (more about the [Manifest format](https://doc.rust-lang.org/cargo/reference/manifest.html)):

```toml [1-5|7-8]
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Rainer Stropek"]
edition = "2018" # Rust edition (default is 2015, current is 2018)

[dependencies]
# No dependencies yet, will follow in a minute
```
<!-- .element: class="fragment" -->

---

## *Cargo.lock*

* Similar <!-- .element: class="fragment" --> to npm's *package-lock.json*
* Maintained <!-- .element: class="fragment" --> by Cargo, you write *Cargo.toml*
* Should <!-- .element: class="fragment" --> you check in *Cargo.lock*?
  * End products: **Yes** (to ensure consistent builds)
  * Libraries: **No**
  * [Read more...](https://doc.rust-lang.org/cargo/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries)

---

## Hands-on Lab

> [RegEx Date Checker](https://github.com/rstropek/CargoIntro/tree/master/samples/01-intro)

* *Cargo* basics (creating a Rust binary program, compiling, running)
* Various Rust language fundamentals
* Basic input (*STDIN*) and output (*STDOUT*)
* Basics about Regular Expressions in Rust
* Debugging Rust

---

## [Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)

* Collection <!-- .element: class="fragment" --> of one or more packages
  * share dependency resolution (shared *Cargo.lock*)
  * common output directory
  * shared settings (e.g. profiles)
* Example: <!-- .element: class="fragment" --> Workspace for samples for this presentation ([GitHub](https://github.com/rstropek/CargoIntro/blob/master/samples/Cargo.toml))

---

# Dependencies

How to specify dependencies ([read more](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html))<br>
*Crate* = German *Kiste, Holzverschlag*

---

## Dependency Sources

| Source                          | Description |
| ------------------------------- | ----------- |
| Version                         | [crates.io](https://crates.io/) or [custom registry](https://doc.rust-lang.org/cargo/reference/registries.html#using-an-alternate-registry)<br>(Git repo + web API, e.g. [Cloudsmith](https://cloudsmith.com/blog/worlds-first-private-cargo-registry-w-cloudsmith-rust/), [Meuse](https://www.meuse.mcorbin.fr/), [GitHub](https://doc.rust-lang.org/cargo/reference/registries.html#running-a-registry)) |
| `git`                           | Pull git repo and look for crate there |
| `path`                          | Look for create in local folder |
| Multiple sources                | Combine registry version and `git` or `path` |

---

## Dependency Types

| *Cargo.toml* section | Description                                                 |
| -------------------- | ----------------------------------------------------------- |
| `dependencies`       | Regular dependency of your package                          |
| `dev-dependencies`   | Only used for compiling tests,<br>examples, and benchmarks  |
| `build-dependencies` | Dependencies for build scripts<br>(also written in Rust)    |

---

## [Referencing *crates.io*](https://github.com/rstropek/CargoIntro/tree/master/samples/10-crates-deps/Cargo.toml)

```toml [4-7]
[package]
...

[dependencies]
rand = "0.8"
num_cpus = "1.0"
num-format = "0.4"
```

```txt
[dependencies]
rand = "0.8"
 ^       ^
 │       └── Version selector
 └── Crate name
```
<!-- .element: class="fragment" -->

---

## Referencing *crates.io*

* Website: <!-- .element: class="fragment" --> [crates.io](https://crates.io/)
  * Example: [`rand`](https://crates.io/crates/rand)
* Guidelines <!-- .element: class="fragment" --> for [publishing crates](https://doc.rust-lang.org/cargo/reference/publishing.html)
  * Sample will follow later
* Version  <!-- .element: class="fragment" --> [references based on SemVer](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) similar to npm's *package.json*
  * Caret <!-- .element: class="fragment" --> requirements: E.g. `^1.2.3` 🡆 `>=1.2.3, <2.0.0`
  * Tilde <!-- .element: class="fragment" --> requirements: E.g. `~1.2.3` 🡆 `>=1.2.3, <1.3.0`
  * Wildcard <!-- .element: class="fragment" --> requirements: E.g. `1.*` 🡆 `>=1.0.0, <2.0.0`
  * Comparison <!-- .element: class="fragment" --> requirements: E.g. `>= 1.2.0`
  * Caret <!-- .element: class="fragment" --> is default: E.g. `1` 🡆 `>=1.0.0, <2.0.0`

---

### *crates.io* Behind the Scenes

* [Package index on GitHub](https://github.com/rust-lang/crates.io-index)
  * Example [*mth_calc*](https://github.com/rust-lang/crates.io-index/blob/master/mt/h_/mth_calc) (sample from this session)
* RESTful Web API
  * [https://crates.io/api/v1/crates](https://crates.io/api/v1/crates)
  * Example: [Query for *mth_calc*](https://crates.io/api/v1/crates?page=1&per_page=10&q=mth_)

---

## Hands-on Lab

> [PI Monte Carlo With *creates.io*](https://github.com/rstropek/CargoIntro/tree/master/samples/10-crates-deps)

* Working with *crates.io* dependencies
* Various Rust language fundamentals
* Basics about working with threads
* Basics about generating random numbers

---

## [Path Dependencies](https://github.com/rstropek/CargoIntro/tree/master/samples/11-crates-deps-folder/Cargo.toml)

```toml [4,7]
[package]
...

[dependencies]
rand = "0.8"
num-format = "0.4"
mth_calc = { path = "mth_calc" }
```

```txt
.
├── Cargo.toml ───────┐
├── src/              │
│     Source code     │
└── mth_calc/   <─────┘
    ├── src/
    │   └── lib.rs
    └── Cargo.toml
```

---

## Hands-on Lab

> [PI Monte Carlo With Library and Path Dependency](https://github.com/rstropek/CargoIntro/tree/master/samples/11-crates-deps-folder)

* Working with *path dependencies*
* Various Rust language fundamentals
* Basics about creating Rust libraries
* Basics about unit testing

---

## [Git Dependencies](https://github.com/rstropek/CargoIntro/tree/master/samples/12-crates-deps-git/Cargo.toml)

* Depend on a library located in a git repository
* You <!-- .element: class="fragment" --> can use `rev`, `tag`, or `branch` to specify exact source location

```toml [4,7]
[package]
...

[dependencies]
rand = "0.8"
num-format = "0.4"
mth_calc = { git = "https://github.com/rstropek/mth-calc" }
```
<!-- .element: class="fragment" -->

---

## Hands-on Lab

> [PI Monte Carlo With Library and Git Dependency](https://github.com/rstropek/CargoIntro/tree/master/samples/12-crates-deps-git)

* Working with *Git dependencies*

---

## Features (1/2)

Optional dependencies (e.g. [`num-format`](https://crates.io/crates/num-format#extra-features), [`serde`](https://serde.rs/feature-flags.html), [`regex`](https://docs.rs/regex/1.4.6/regex/#crate-features))

```toml [5|6]
[package]
...

[dependencies]
serde = { version = "1.0", features = ["derive"] }
regex = { version = "1", default-features = false, features = [ "std", "unicode-perl" ]}
```

---

## Features (2/2)

Conditional compilation

```toml [2,3|6]
[features]
default = ["console_error_panic_hook", "console_log", "wee_alloc"]
console_log = ["web-sys"]

[dependencies]
web-sys = { version = "0.3.50", features = ["console"], optional = true }
```

```rs [1-2|6,7]
#[cfg(feature = "console_log")]
use web_sys::console;

pub fn do_something() {
  ...
  #[cfg(feature = "console_log")]
  console::log_1(&"Hello World".into());
  ...
}
```
<!-- .element: class="fragment" -->

---

## Advanced Dependency Topics

Not covered in detail here

* Multiple <!-- .element: class="fragment" --> locations
  * Specify both a registry version **and** a *git* or *path* location
  * `mth_calc = { path = "mth_calc", version = "0.1" }`
  * Use case: Lib splitted up into multiple packages within workspace
    * Locally: Use paths
    * When published: Use *crates.io* versions
* Overrides <!-- .element: class="fragment" -->
  * Work with a crate before it has been published ([docs](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html))
* Platform-specific <!-- .element: class="fragment" --> dependencies
  * E.g. `[target.'cfg(windows)'.dependencies]` ([docs](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#platform-specific-dependencies))

---

## Advanced Dependency Topics (Continued)

* Build <!-- .element: class="fragment" --> scripts ([docs](https://doc.rust-lang.org/cargo/reference/build-scripts.html))
  * Use cases e.g. build C libraries, generate Rust code, set platform-specif config settings, etc.
  * *build.rs* in the root of a package 🡆 compiled and executed before building the package

---

# Publishing Crates

How to publish crates ([read more](https://doc.rust-lang.org/cargo/reference/publishing.html))

---

## Important Cargo Command for Publishing

| Command                                                                         | Description                                                    |
| ------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| [`cargo login`](https://doc.rust-lang.org/cargo/commands/cargo-login.html)      | Save an API token from the registry (e.g. *crates.io*) locally |
| [`cargo package`](https://doc.rust-lang.org/cargo/commands/cargo-package.html)  | Assemble the local package into a distributable tarball        |
| [`cargo publish `](https://doc.rust-lang.org/cargo/commands/cargo-publish.html) | Upload a package to the registry                               |

---

## Checklist

* Get <!-- .element: class="fragment" --> API token from *crates.io* ([details](https://doc.rust-lang.org/cargo/reference/publishing.html#before-your-first-publish))
  * Run `cargo login <your_token>` with it
* Put <!-- .element: class="fragment" --> meaningful metadata into your *Cargo.toml*
  * Add *README.md*
* Add <!-- .element: class="fragment" --> unit tests, integration tests, and benchmarks
* Document <!-- .element: class="fragment" --> API ([docs](https://doc.rust-lang.org/stable/rust-by-example/meta/doc.html))
  * Include examples

```bash [1,2|4,5|7,8]
cargo publish --dry-run
# Check package in target/package

# Verify proper content of package (avoid unnecessary files)
cargo package --list

# Upload to crates.io
cargo publish
```
<!-- .element: class="fragment" -->

---

## Hands-on Lab

> [Publish PI Monte Carlo libary to *crates.io*](https://github.com/rstropek/CargoIntro/tree/master/samples/30-crate)

* Add documentation to your package
  * Including example code
* Publish create to *crates.io*
* See result [on *crates.io*](https://crates.io/crates/mth_calc) and [*docs.rs*](https://docs.rs/mth_calc/0.1.1/mth_calc/)

---

## Hands-on Lab

```bash [1-3|5-6|8-9]
# Build documentation
cargo doc
# Inspect docs in *target/doc*

# Run tests
cargo test

# Test samples in docs
cargo test --doc
```

---

# Summary

* Cargo <!-- .element: class="fragment" --> is an indispensible for Rust development
* Many <!-- .element: class="fragment" --> similarities with *npm*
* Unexpected <!-- .element: class="fragment" --> features for me as someone who is new to Rust
  * Documentation <!-- .element: class="fragment" --> features
  * Great <!-- .element: class="fragment" --> support for examples (in docs, in *examples* folder)
  * Benchmarking <!-- .element: class="fragment" --> built-in
* Room <!-- .element: class="fragment" --> for improvement
  * Make it easier to get a custom package repository

---

# Thank You for Attending

![Rust Linz](https://rust-linz.at/img/rust-linz-logo.svg)

Rainer Stropek | @rstropek | Coding Club Linz
{"mode":"full","isActive":false}