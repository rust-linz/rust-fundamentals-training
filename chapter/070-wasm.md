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

- Memory safety
- Powerful language
  - Macros
  - Traits
  - Enums
- Fast (zero-cost abstractions, no runtime)
- Reuse code on server and client
- Share code between platforms (e.g. desktop, mobile)

---

## Rust and WebAssembly

- [Rust and WASM intro](https://rustwasm.github.io/docs/book/introduction.html)
- [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/introduction.html)
- [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/)
  - [*web_sys* crate](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) (e.g. interact with DOM from RUST)

---

## Step-by-Step Example: 🎇 Fireworks

> In separate [GitHub repository](https://github.com/rstropek/rust-samples/tree/master/fireworks)

- Learn features of Rust WASM in a larger example
  - [Try the final example](https://cddataexchange.blob.core.windows.net/data-exchange/fireworks/index.html)
- Use VSCode snippets to walk through the sample
