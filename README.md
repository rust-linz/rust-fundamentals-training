# Rust Fundamentals Training

## Preqrequisites

Before the workshop, make sure you have the correct tools installed and verified. You need:

1. [Rustup Toolchain](https://rustup.rs/)
2. An editor of your choice (we highly recommmend [VSCode](https://code.visualstudio.com/) + Extensions)

## Install and verify Rust

[Rustup](https://rustup.rs) provides you with all the software to compile and run Rust applications, e.g.

1. Cargo - build tool and package manager
2. `rustfmt` - Auto-formatting tool for Rust code
3. `clippy` - Linting for common mistakes

[and many more](https://rust-lang.github.io/rustup-components-history/). *Rustup* also allows you to install different compile targets and multiple toolchains, as well as keeping your toolchains up to date.

After installing, you should have a set of new command line tools available. 

Verify your *Rust* installation:

1. Open a Terminal/Shell of your choice
2. Navigate to a folder you want to use for your Rust projects
3. Enter

```bash
$ cargo new installation-test
```

4. Cargo will create a "Hello World" application for you. Enter the newly created directory

```bash
$ cd installation-test
```

5. Build and run

```bash
$ cargo run
```

If you see compile information and `Hello, world!` printed out on your command line, you are ready to go!

## Tooling with VSCode

Install [VS Code](https://code.visualstudio.com) for your platform.

We recommend to use the following extensions:

1. [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
2. [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) for debugging
