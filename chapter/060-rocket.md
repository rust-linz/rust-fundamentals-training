# Rocket

## Building Web APIs with 🚀 Rocket

---

## Web Server Framework

- Lots of frameworks for web development ([see more](https://github.com/rust-unofficial/awesome-rust#web-programming))
- We use [Rocket](https://rocket.rs)
  - Fast and simple (by using powerful macros)
  - Type safe
  - Less boilerplate
  - Extensible (based on Traits)

---

## Step-by-Step Example

> In separate [GitHub repository](https://github.com/rstropek/RustyRockets)

- Learn features of Rocket based on series of small samples
- Already based on v0.5 (at the time of writing available as RC 1)
- Use VSCode snippets to walk through the sample

---

## Included Features

- [Routing](https://rocket.rs/v0.5-rc/guide/overview/#routing) (based on macros and parameter types)
  - With [dynamic paths](https://rocket.rs/v0.5-rc/guide/requests/#dynamic-paths)
  - With [query strings](https://rocket.rs/v0.5-rc/guide/requests/#query-strings)
- [Async routes](https://rocket.rs/v0.5-rc/guide/overview/#async-routes)
- [Request guards](https://rocket.rs/v0.5-rc/guide/requests/#request-guards) for e.g. security checks, extracting [cookies](https://rocket.rs/v0.5-rc/guide/requests/#cookies), etc.
- Support for different body [formats](https://rocket.rs/v0.5-rc/guide/requests/#format) (e.g. JSON, [forms](https://rocket.rs/v0.5-rc/guide/requests/#forms))
- [Error catcher](https://rocket.rs/v0.5-rc/guide/requests/#error-catchers) (e.g. 404 error)
- Extensible [response encoding](https://rocket.rs/v0.5-rc/guide/responses/)

---

## Included Features (continued)

- [Managed state](https://rocket.rs/v0.5-rc/guide/state/)
  - With support for various [databases](https://rocket.rs/v0.5-rc/guide/state/#databases) included
- [Fairings](https://rocket.rs/v0.5-rc/guide/fairings/) (similar to middlewares in other platforms)
- Possibilities to implement [integration tests](https://rocket.rs/v0.5-rc/guide/testing/)
  - Support for mocking state management data types
