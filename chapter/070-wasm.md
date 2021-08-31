# Rust and WebAssembly

## Running Rust in the Browser

---

## [WebAssembly](https://webassembly.org/)

- Simple <!-- .element: class="fragment" --> machine model (VM)
- For <!-- .element: class="fragment" --> browser and outside browser
- WAT <!-- .element: class="fragment" --> ([WebAssemblyText](https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format)) -> WASM files (like ELF)
- Linear <!-- .element: class="fragment" --> memory model (flat array of bytes)

---

## [Use Cases](https://webassembly.org/docs/use-cases/)

- Inside <!-- .element: class="fragment" --> the browser
  - CPU-intensive workloads (e.g. games, CAD)
  - Reuse existing code
- Outside <!-- .element: class="fragment" --> the browser
  - Server-side execution of untrusted or semi-trusted code
  - Reuse code with browser apps

---

## Why Rust in the Browser?

- Memory <!-- .element: class="fragment" --> safety
- Powerful <!-- .element: class="fragment" --> language
  - Macros
  - Traits
  - Enums
- Fast <!-- .element: class="fragment" --> and small (zero-cost abstractions, no runtime)
- Reuse <!-- .element: class="fragment" --> code on server and client
- Share <!-- .element: class="fragment" --> code between platforms (e.g. desktop, mobile)

---

## Rust and WebAssembly

- [Rust and WASM intro](https://rustwasm.github.io/docs/book/introduction.html)
- [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/introduction.html)
- [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/)
  - [*web_sys* crate](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) (e.g. interact with DOM from RUST)

---

## Step-by-Step Example: Rust Wasm 101

[Code on GitHub](examples/110-wasm)

- Learn features of Rust WASM step-by-step
- Uses TypeScript for web client

---

## Step-by-Step Example: 🎇 Fireworks

> In separate [GitHub repository](https://github.com/rstropek/rust-samples/tree/master/fireworks)

- Learn features of Rust WASM in a larger example
  - [Try the final example](https://cddataexchange.blob.core.windows.net/data-exchange/fireworks/index.html)
- Use VSCode snippets to walk through the sample

---

## WASI

### WebAssembly Systems Interface

> WebAssembly is an assembly language for a conceptual machine, not a physical one.

Lin Clark, [source](https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/)

---

![NON WASI](https://hacks.mozilla.org/files/2019/03/04-01-portability-1-500x375.png)

[source](https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/)

---

![WASI](https://hacks.mozilla.org/files/2019/03/04-02-portability-1-500x484.png)

[source](https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/)

---

## Why?

- Portability. A true, multi-platform VM
- Security. WASM is per default sandboxed. Hosts can decide what to allow and what not

Hello containers?

---

## WASI in Rust

- Install the right toolchain: `rustup target add wasm32-wasi`
- Let cargo build the right target `cargo build --target wasm32-wasi --release`
- Run with `wasmtime``

---

## WASI with Cargo

- `cargo install cargo-wasi` -- A Cargo plug-in!
- `cargo wasi build --release`
- `cargo wasi run`

---

## Resources

- [Bytecode Alliance](https://bytecodealliance.org/)
- [Krustlet](https://krustlet.dev/)
- [WAGI](https://deislabs.io/posts/introducing-wagi-easiest-way-to-build-webassembly-microservices/)

---

> WebAssembly: Neither Web, Nor Assembly, but Revolutionary

-- Jay Phelps [source](https://www.javascriptjanuary.com/blog/webassembly-neither-web-nor-assembly-but-revolutionary)